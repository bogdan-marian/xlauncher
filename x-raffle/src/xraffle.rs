#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod constant;
pub mod event;
pub mod data;
pub mod storage;
pub mod user;
pub mod admin;
pub mod view;
pub mod config;

#[elrond_wasm::contract]
pub trait XRaffle:
    storage::StorageModule
    + event::EventModule
    + user::UserModule
    + admin::AdminModule
    + config::ConfigModule
    + view::ViewModule
{
    #[init]
    fn init(&self) {}
}
