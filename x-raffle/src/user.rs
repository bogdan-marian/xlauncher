elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::module]
pub trait UserModule:
    crate::storage::StorageModule
    + crate::event::EventModule
{
}