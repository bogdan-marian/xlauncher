#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod config;
mod event;
mod state;
mod storage;
mod user;
mod admin;
mod view;

#[elrond_wasm::contract]
pub trait XRaffle:
    storage::StorageModule
    + event::EventModule
    + user::UserModule
    + admin::AdminModule
    + view::ViewModule
{
    #[init]
    fn init(&self) {}
}
