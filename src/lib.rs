// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/*!
# `walrus-meter`

This crate provides instrumentation-based metering for Wasm modules using the
`walrus` library.

Provide the name of a function to call, and the function will be called
immediately before executing a sequence of instructions with that sequence's
cost.
*/

pub mod costs;

#[doc(inline)]
pub use costs::Costs;

use std::collections::HashMap;

/// The type of errors raised by this crate.
pub type Error = anyhow::Error;

#[derive(Default)]
struct InfoItem {
    // The minimum cost of entering this sequence (after any prepayment)
    min_cost: usize,
    // The number of enclosing blocks this sequence can escape
    escape_velocity: usize,
}

struct Instrument<C> {
    costs: C,
    spend: walrus::FunctionId,
    info: Info,
    // Walrus IR doesn't preserve the relative block indices, so we have to
    // reconstruct them by keeping a stack.
    stack: Vec<(walrus::ir::InstrSeqId, InfoItem)>,
}

#[derive(Default)]
struct Info {
    items: HashMap<walrus::ir::InstrSeqId, InfoItem>,
}

#[facile::facade]
impl InstrExt for walrus::ir::Instr {
    fn jump_targets(&self) -> Vec<walrus::ir::InstrSeqId> {
        use walrus::ir::Instr::*;
        match self {
            Br(walrus::ir::Br { block })
            | BrIf(walrus::ir::BrIf { block })
            | BrOnCast(walrus::ir::BrOnCast { block, .. })
            | BrOnCastFail(walrus::ir::BrOnCastFail { block, .. })
            | BrOnNull(walrus::ir::BrOnNull { block })
            | BrOnNonNull(walrus::ir::BrOnNonNull { block }) => vec![*block],
            BrTable(walrus::ir::BrTable { blocks, default }) => blocks
                .iter()
                .cloned()
                .chain(std::iter::once(*default))
                .collect(),
            _ => vec![],
        }
    }

    fn branches(&self) -> Vec<walrus::ir::InstrSeqId> {
        use walrus::ir::Instr::*;

        match self {
            IfElse(walrus::ir::IfElse {
                consequent,
                alternative,
            }) => vec![*consequent, *alternative],

            Block(walrus::ir::Block { seq }) => vec![*seq],

            _ => vec![],
        }
    }

    fn entry_points(&self) -> Vec<walrus::ir::InstrSeqId> {
        if let walrus::ir::Instr::Loop(walrus::ir::Loop { seq }) = self {
            vec![*seq]
        } else {
            self.branches()
        }
    }
}

impl Info {
    fn can_escape(&self, instr: &walrus::ir::Instr) -> bool {
        !instr.jump_targets().is_empty()
            || instr
                .entry_points()
                .iter()
                .map(|seq_id| self.items.get(seq_id).unwrap().escape_velocity)
                .max()
                .unwrap_or_default()
                > 1
    }
}

struct BasicBlocks<'info, 'instrs> {
    info: &'info Info,
    instrs: &'instrs [(walrus::ir::Instr, walrus::InstrLocId)],
}

impl<'info, 'instrs> Iterator for BasicBlocks<'info, 'instrs> {
    type Item = &'instrs [(walrus::ir::Instr, walrus::InstrLocId)];

    fn next(&mut self) -> Option<Self::Item> {
        if self.instrs.is_empty() {
            None
        } else {
            let pos = self
                .instrs
                .iter()
                .position(|(instr, _loc)| self.info.can_escape(instr))
                .map_or(self.instrs.len(), |pos| pos + 1);
            let ret = &self.instrs[..pos];
            self.instrs = &self.instrs[pos..];
            Some(ret)
        }
    }
}

impl<C: Costs> Instrument<C> {
    fn new(costs: C, spend: walrus::FunctionId) -> Self {
        Self {
            costs,
            spend,
            info: Info::default(),
            stack: vec![],
        }
    }

    fn enter_seq(&mut self, id: walrus::ir::InstrSeqId) {
        self.stack.push((id, InfoItem::default()))
    }

    fn exit_seq(&mut self) -> (walrus::ir::InstrSeqId, InfoItem) {
        self.stack.pop().expect("enter should be paired with exit")
    }

    fn info(&self, seq: walrus::ir::InstrSeqId) -> &InfoItem {
        self.info.items.get(&seq).unwrap()
    }

    fn relative_label(&self, seq: walrus::ir::InstrSeqId) -> usize {
        self.stack
            .iter()
            .rev()
            .position(|(s, _info)| *s == seq)
            .expect("Wasm should be well-formed")
    }

    fn escape_velocity(&self, instr: &walrus::ir::Instr) -> usize {
        if instr.is_return()
            || instr.is_return_call_indirect()
            || instr.is_return_call()
            || instr.is_return_call_ref()
        {
            self.stack.len()
        } else {
            std::cmp::max(
                instr
                    .entry_points()
                    .into_iter()
                    .map(|seq| self.info(seq).escape_velocity)
                    .max()
                    .unwrap_or_default()
                    .saturating_sub(1),
                instr
                    .jump_targets()
                    .into_iter()
                    .map(|seq| self.relative_label(seq) + 1)
                    .max()
                    .unwrap_or_default(),
            )
        }
    }

    fn basic_blocks<'instr>(
        &self,
        seq: &'instr [(walrus::ir::Instr, walrus::InstrLocId)],
    ) -> impl Iterator<Item = &'instr [(walrus::ir::Instr, walrus::InstrLocId)]> {
        BasicBlocks {
            info: &self.info,
            instrs: seq,
        }
    }

    fn extract_min_cost(&mut self, instr: &walrus::ir::Instr) -> usize {
        let nested_cost = instr
            .branches()
            .iter()
            .map(|seq| self.info(*seq).min_cost)
            .min()
            .unwrap_or_default();
        for nest in instr.branches() {
            self.info.items.get_mut(&nest).unwrap().min_cost -= nested_cost;
        }
        self.costs.instruction(instr) as usize + nested_cost
    }

    fn call_spend(&self, output: &mut Vec<(walrus::ir::Instr, walrus::InstrLocId)>, cost: usize) {
        let cost = i64::try_from(cost).unwrap();
        if cost > 0 {
            output.push((
                walrus::ir::Const {
                    value: walrus::ir::Value::I64(cost),
                }
                .into(),
                walrus::InstrLocId::default(),
            ));

            output.push((
                walrus::ir::Call { func: self.spend }.into(),
                walrus::InstrLocId::default(),
            ));
        }
    }
}

// Build up information about the function.
impl<'instr, C: Costs> walrus::ir::Visitor<'instr> for Instrument<C> {
    fn start_instr_seq(&mut self, seq: &'instr walrus::ir::InstrSeq) {
        self.enter_seq(seq.id());
    }

    fn end_instr_seq(&mut self, seq: &'instr walrus::ir::InstrSeq) {
        let escape_velocity = seq
            .iter()
            .map(|(instr, _loc)| self.escape_velocity(instr))
            .max()
            .unwrap_or_default();

        let mut min_cost = 0;
        for (instr, _loc) in self
            .basic_blocks(seq)
            .take(1)
            .flatten()
            .collect::<Vec<_>>()
        {
            min_cost += self.extract_min_cost(instr);
        }

        let (id, mut info) = self.exit_seq();
        debug_assert_eq!(id, seq.id());

        info.escape_velocity = escape_velocity;
        info.min_cost = min_cost;

        self.info.items.insert(id, info);
    }
}

impl<C: Costs> walrus::ir::VisitorMut for Instrument<C> {
    fn end_instr_seq_mut(&mut self, seq: &mut walrus::ir::InstrSeq) {
        let seq_min_cost = self.info(seq.id()).min_cost;
        for (index, basic_block) in self
            .basic_blocks(&std::mem::take(&mut seq.instrs))
            .collect::<Vec<_>>()
            .into_iter()
            .enumerate()
        {
            let cost = if index == 0 {
                seq_min_cost
            } else {
                basic_block
                    .iter()
                    .map(|(instr, _loc)| self.extract_min_cost(instr))
                    .sum()
            };

            self.call_spend(&mut seq.instrs, cost);
            seq.instrs.extend_from_slice(basic_block);
        }
    }
}

/// Injects the named spend function and instrument all functions to call it
/// according to the given costs.
pub fn instrument(
    module: &[u8],
    costs: impl Costs,
    (spend_module, spend_name): (&str, &str),
) -> Result<Vec<u8>, Error> {
    let mut module = walrus::Module::from_buffer(module)?;
    let spend_ty = module.types.add(&[walrus::ValType::I64], &[]);
    let (spend, _) = module.add_import_func(spend_module, spend_name, spend_ty);

    for (_id, func) in module.funcs.iter_local_mut() {
        let mut visitor = Instrument::new(&costs, spend);
        walrus::ir::dfs_in_order(&mut visitor, func, func.entry_block());
        walrus::ir::dfs_pre_order_mut(&mut visitor, func, func.entry_block());
    }

    Ok(module.emit_wasm())
}
