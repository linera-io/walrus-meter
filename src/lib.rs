pub mod costs;

/// A trait that encodes the costs of different Wasm operations.
#[auto_impl::auto_impl(&)]
pub trait Costs {
    fn instruction(&self, instruction: &walrus::ir::Instr) -> i32;
}

struct Instrument<C> {
    costs: C,
    spend: walrus::FunctionId,
}

impl<C: Costs> walrus::ir::VisitorMut for Instrument<C> {
    fn start_instr_seq_mut(&mut self, instr_seq: &mut walrus::ir::InstrSeq) {
        fn is_control_flow(instr: &walrus::ir::Instr) -> bool {
            // We consider blocks to be control-flow structures as they may
            // contain control-flow structures. For efficiency, we could check
            // whether the block contains a control-flow structure, but to do so
            // we would need a reference to the containing `LocalFunction`,
            // which we can't get as we're currently traversing it mutably.
            use walrus::ir::Instr::*;
            matches!(instr, Br(_) | BrIf(_) | BrTable(_) | Return(_) | Block(_) | Loop(_) | IfElse(_))
        }

        let mut insert_positions: Vec<(usize, i64)> = Vec::new();
        let mut current_cost: i64 = 0;
        let mut basic_block_start: usize = 0;

        for (idx, (instr, _)) in instr_seq.instrs.iter().enumerate() {
            current_cost += self.costs.instruction(instr) as i64;

            if is_control_flow(instr) {
                insert_positions.push((basic_block_start, current_cost));
                basic_block_start = idx + 1;
                current_cost = 0;
            }
        }

        if basic_block_start != instr_seq.instrs.len() {
            insert_positions.push((basic_block_start, current_cost));
        }

        // Insert in reverse order to preserve indices
        for (pos, cost) in insert_positions.into_iter().rev() {
            instr_seq.instrs.insert(
                pos,
                (
                    walrus::ir::Call { func: self.spend }.into(),
                    walrus::InstrLocId::default(),
                ),
            );
            instr_seq.instrs.insert(
                pos,
                (
                    walrus::ir::Const { value: walrus::ir::Value::I64(cost) }.into(),
                    walrus::InstrLocId::default(),
                ),
            );
        }
    }
}

pub type Error = anyhow::Error;

pub fn instrument(
    module: &[u8],
    costs: impl Costs,
    (spend_module, spend_name): (&str, &str),
) -> Result<Vec<u8>, Error> {
    let mut module = walrus::Module::from_buffer(module)?;
    let spend_ty = module.types.add(&[walrus::ValType::I64], &[]);
    let (spend, _) = module.add_import_func(spend_module, spend_name, spend_ty);

    for (_id, func) in module.funcs.iter_local_mut() {
        walrus::ir::dfs_pre_order_mut(
            &mut Instrument { costs: &costs, spend },
            func,
            func.entry_block(),
        );
    }

    Ok(module.emit_wasm())
}
