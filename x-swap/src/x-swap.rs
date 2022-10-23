#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod swap_data;


#[elrond_wasm::derive::contract]
pub trait XSwap {
    #[init]
    fn init(&self) {}


    // storage
    #[storage_mapper("pairRoyaltyPercentage")]
    fn pair_royalty_percentage(
        &self,
        egld_or_esdt_identifier: &EgldOrEsdtTokenIdentifier,
    ) -> SingleValueMapper<BigUint>;

    #[storage_mapper("defaultRoyaltyPercentage")]
    fn default_royalty_percentage(&self) -> SingleValueMapper<BigUint>;

    #[view(getWhiteList)]
    #[storage_mapper("whiteList")]
    fn white_list(&self) -> UnorderedSetMapper<EgldOrEsdtTokenIdentifier>;
}



