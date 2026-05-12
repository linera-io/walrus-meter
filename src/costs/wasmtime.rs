// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/**
Costs that aim to be roughly compatible with Wasmtime's defaults.

<div class="warning">
In some cases these will not yield the same results as Wasmtime,
as Wasmtime will charge for the implicit return at the end of a function,
while this does not.
</div>
*/
pub struct Costs;
impl super::Costs for Costs {
    fn instruction(&self, instruction: &walrus::ir::Instr) -> i32 {
        use walrus::ir::Instr::*;
        match instruction {
            Drop(_) | Block(_) | Loop(_) | Unreachable(_) => 0,
            _ => 1,
        }
    }
}

#[cfg(test)]
mod test {
    use wasm_instrument::parity_wasm;
    use test_case::test_case;

    impl wasm_instrument::gas_metering::Rules for super::Costs {
        fn instruction_cost(
            &self,
            instruction: &parity_wasm::elements::Instruction,
        ) -> Option<u32> {
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

    fn parity_instrument(module: &[u8]) -> anyhow::Result<Vec<u8>> {
        Ok(wasm_instrument::gas_metering::inject(
            parity_wasm::deserialize_buffer(module)?,
            wasm_instrument::gas_metering::host_function::Injector::new("host", "spend"),
            &super::Costs,
        )
        .map_err(|_| anyhow::anyhow!("injection failed"))?
        .into_bytes()?)
    }

    #[test_case(include_str!("fixtures/branch.wat"); "branch")]
    #[test_case(include_str!("fixtures/call.wat"); "call")]
    #[test_case(include_str!("fixtures/ifs.wat"); "ifs")]
    #[test_case(include_str!("fixtures/loop.wat"); "loop_")]
    #[test_case(include_str!("fixtures/simple.wat"); "simple")]
    fn wasm_instrument_equivalent(code: &str) -> wasmtime::Result<()> {
        use wasmtime::ToWasmtimeResult as _;

        let engine = wasmtime::Engine::default();
        let mut store = wasmtime::Store::new(&engine, 0i64);
        let mut linker = wasmtime::Linker::new(&engine);
        linker.func_wrap(
            "host",
            "spend",
            |mut caller: wasmtime::Caller<'_, i64>, cost: i64| {
                *caller.data_mut() += cost;
            },
        )?;

        let module = wat::parse_str(code)?;

        let parity_instrument_cost = {
            *store.data_mut() = 0;
            linker.instantiate(
                &mut store,
                &wasmtime::Module::from_binary(
                    &engine,
                    &parity_instrument(&module).to_wasmtime_result()?,
                )?,
            )?;
            *store.data() as u64
        };

        let walrus_instrument_cost = {
            *store.data_mut() = 0;
            linker.instantiate(
                &mut store,
                &wasmtime::Module::from_binary(
                    &engine,
                    &crate::instrument(&module, super::Costs, ("host", "spend"))
                        .to_wasmtime_result()?,
                )?,
            )?;
            *store.data() as u64
        };

        assert_eq!(
            walrus_instrument_cost, parity_instrument_cost,
            "\n\
             \twalrus-meter cost:    {walrus_instrument_cost}\n\
             \twasm-instrument cost: {parity_instrument_cost}",
        );

        Ok(())
    }
}
