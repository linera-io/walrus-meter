// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/*!
Various prebuilt cost strategies.
*/

mod wasmtime;
pub use wasmtime::Costs as Wasmtime;

/// A trait that encodes the costs of different Wasm operations.
#[auto_impl::auto_impl(&, &mut, box)]
pub trait Costs {
    fn instruction(&self, instruction: &walrus::ir::Instr) -> i32;

    // TODO: add an implicit_return cost that can be used to instrument the
    // module to achieve Wasmtime equivalence.

    // TODO: achieve feature parity with `wasm-instrument`, with memory-grow
    // costs and per-local costs.
}

/// Costs that never consume fuel.
pub struct Free;
impl Costs for Free {
    fn instruction(&self, _instruction: &walrus::ir::Instr) -> i32 {
        0
    }
}
