#![no_std]


elrond_wasm::imports!();

#[elrond_wasm::derive::contract]
pub trait XSwap{
    #[init]
    fn init(&self){

    }
}
