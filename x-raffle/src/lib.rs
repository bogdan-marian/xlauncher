#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod config;
mod event;
mod state;
mod storage;
mod user;
mod admin;

#[elrond_wasm::contract]
pub trait XRaffle:
    storage::StorageModule
    + event::EventModule
    + user::UserModule
    + admin::AdminModule
{
    #[init]
    fn init(&self) {}
}
