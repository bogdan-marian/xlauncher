#![no_std]

elrond_wasm::imports!();

pub mod config;
pub mod data;
pub mod storage;
pub mod event;
pub mod user;
pub mod admin;
pub mod view;

#[elrond_wasm::contract]
pub trait XSwap:
    storage::StorageModule
    + event::EventModule
    + user::UserModule
    + admin::AdminModule
    + view::ViewModule
{
    #[init]
    fn init(&self) {}
}
