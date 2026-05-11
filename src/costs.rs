// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// Costs that never consume fuel.
pub struct Free;
impl super::Costs for Free {
    fn instruction(&self, _instruction: &walrus::ir::Instr) -> i32 {
        0
    }
}

/// Costs that aim to be compatible with Wasmtime's defaults.
pub struct Wasmtime;

impl super::Costs for Wasmtime {
    fn instruction(&self, instruction: &walrus::ir::Instr) -> i32 {
        use walrus::ir::Instr::*;
        match instruction {
            Drop(_) | Block(_) | Loop(_) | Unreachable(_) => 0,
            _ => 1,
        }
    }
}

#[test]
fn test_wasmtime_equivalence() -> anyhow::Result<()> {
    let engine = wasmtime::Engine::new(&*wasmtime::Config::new().consume_fuel(true))?;
    let mut store = wasmtime::Store::new(&engine, 0i64);
    let mut linker = wasmtime::Linker::new(&engine);
    linker.func_wrap(
        "host",
        "spend",
        |mut caller: wasmtime::Caller<'_, i64>, cost: i64| {
            *caller.data_mut() += cost;
        },
    )?;

    let module = std::fs::read("fixtures/loop.wasm")?;

    let count = 5;

    let wasmtime_cost = {
        store.set_fuel(10_000)?;
        linker
            .instantiate(
                &mut store,
                &wasmtime::Module::from_binary(&engine, &module)?,
            )?
            .get_typed_func::<u32, ()>(&mut store, "run")?
            .call(&mut store, count)?;
        10_000 - store.get_fuel()?
    };

    let walrus_instrument_cost = {
        *store.data_mut() = 0;
        linker
            .instantiate(
                &mut store,
                &wasmtime::Module::from_binary(
                    &engine,
                    &super::instrument(&module, Wasmtime, ("host", "spend"))?,
                )?,
            )?
            .get_typed_func::<u32, ()>(&mut store, "run")?
            .call(&mut store, count)?;
        *store.data() as u64
    };

    assert_eq!(walrus_instrument_cost, wasmtime_cost);

    Ok(())
}
