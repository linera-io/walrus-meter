// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/*!
Various prebuilt cost strategies.
*/

mod wasmtime;
pub use wasmtime::Costs as Wasmtime;

/// Costs that never consume fuel.
pub struct Free;
impl super::Costs for Free {
    fn instruction(&self, _instruction: &walrus::ir::Instr) -> i32 {
        0
    }
}
