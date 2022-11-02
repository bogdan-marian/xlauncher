#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait XSwap
{
    #[init]
    fn init(&self) {}
}
