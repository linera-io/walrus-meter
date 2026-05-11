use wasm_instrument::parity_wasm;

trait Rules {
    fn instruction_cost(&self, instruction: &walrus::ir::Instr) -> i32;
}

struct WasmtimeRules;

impl Rules for WasmtimeRules {
    fn instruction_cost(&self, instruction: &walrus::ir::Instr) -> i32 {
        use walrus::ir::Instr::*;
        match instruction {
            // Nop?
            Drop(_) | Block(_) | Loop(_) | Unreachable(_) | Return(_) | IfElse(_) | BrIf(_) | Call(_) => 0,
            x => {
                println!("counting instruction: {x:?}");
                1
            },
        }
    }
}

impl wasm_instrument::gas_metering::Rules for WasmtimeRules {
    fn instruction_cost(&self, instruction: &parity_wasm::elements::Instruction) -> Option<u32> {
        use parity_wasm::elements::Instruction::*;

        Some(match instruction {
            Nop | Drop | Block(_) | Loop(_) | Unreachable | Else | End => 0,
            _ => 1,
        })
    }

    fn memory_grow_cost(&self) -> wasm_instrument::gas_metering::MemoryGrowCost {
        wasm_instrument::gas_metering::MemoryGrowCost::Free
    }

    fn call_per_local_cost(&self) -> u32 {
        0
    }
}

struct Instrument<R> {
    rules: R,
    spend: walrus::FunctionId,
}

impl<R: Rules> walrus::ir::VisitorMut for Instrument<R> {
    fn start_instr_seq_mut(&mut self, instr_seq: &mut walrus::ir::InstrSeq) {
        let cost: i64 = instr_seq.instrs.iter().map(|(instr, _loc_id)| self.rules.instruction_cost(instr) as i64).sum();
        instr_seq.instrs.insert(
            0,
            (
                walrus::ir::Call { func: self.spend }.into(),
                walrus::InstrLocId::default(),
            ),
        );
        instr_seq.instrs.insert(
            0,
            (
                walrus::ir::Const { value: walrus::ir::Value::I64(cost) }.into(),
                walrus::InstrLocId::default(),
            ),
        );
    }
}

fn instrument(module: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut module = walrus::Module::from_buffer(module)?;
    let spend_ty = module.types.add(&[walrus::ValType::I64], &[]);
    let (spend, _) = module.add_import_func("host", "spend", spend_ty);

    for (id, func) in module.funcs.iter_local_mut() {
        println!("{:?}", id);
        walrus::ir::dfs_pre_order_mut(
            &mut Instrument {
                rules: WasmtimeRules,
                spend,
            },
            func,
            func.entry_block(),
        );
    }

    Ok(module.emit_wasm())
}

fn parity_instrument(module: &[u8]) -> anyhow::Result<Vec<u8>> {
    Ok(wasm_instrument::gas_metering::inject(
        parity_wasm::deserialize_buffer(module)?,
        wasm_instrument::gas_metering::host_function::Injector::new(
            "host",
            "spend",
        ),
        &WasmtimeRules,
    ).map_err(|_| anyhow::anyhow!("injection failed"))?.into_bytes()?)
}

fn main() -> anyhow::Result<()> {
    let engine = wasmtime::Engine::new(&*wasmtime::Config::new().consume_fuel(true))?;
    let mut store = wasmtime::Store::new(&engine, 0i64);
    let mut linker = wasmtime::Linker::new(&engine);
    linker.func_wrap("host", "spend", |mut caller: wasmtime::Caller<'_, i64>, cost: i64| {
        *caller.data_mut() += cost;
    })?;

    let module = std::fs::read("target/wasm32-unknown-unknown/debug/examples/loop.wasm")?;
    let count = 0;

    let wasmtime_cost = {
        store.set_fuel(10_000)?;
        linker
            .instantiate(&mut store, &wasmtime::Module::from_binary(&engine, &module)?)?
            .get_typed_func::<u32, ()>(&mut store, "run")?
            .call(&mut store, count)?;
        10_000 - store.get_fuel()?
    };

    let walrus_instrument_cost = {
        *store.data_mut() = 0;
        linker
            .instantiate(
                &mut store,
                &wasmtime::Module::from_binary(&engine, &instrument(&module)?)?,
            )?
            .get_typed_func::<u32, ()>(&mut store, "run")?
            .call(&mut store, count)?;
        *store.data()
    };

    let parity_instrument_cost = {
        *store.data_mut() = 0;
        linker
            .instantiate(
                &mut store,
                &wasmtime::Module::from_binary(&engine, &parity_instrument(&module)?)?,
            )?
            .get_typed_func::<u32, ()>(&mut store, "run")?
            .call(&mut store, count)?;
        *store.data()
    };

    println!(
        "wasmtime cost: {wasmtime_cost}\n\
         walrus-instrument cost: {walrus_instrument_cost}\n\
         wasm-instrument cost: {parity_instrument_cost}",
    );

    assert_eq!(walrus_instrument_cost, parity_instrument_cost);

    Ok(())
}
