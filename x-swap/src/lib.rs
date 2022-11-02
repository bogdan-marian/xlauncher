#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod config;
mod state;
mod storage;
mod event;
mod user;

#[elrond_wasm::contract]
pub trait XSwap:
    storage::StorageModule
    + event::EventModule
    + user::UserModule
{
    #[init]
    fn init(&self) {}
}
