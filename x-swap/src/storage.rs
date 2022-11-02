elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::state::{ Offer };

#[elrond_wasm::module]
pub trait StorageModule
{
    #[view(getRaffleScAddress)]
    #[storage_mapper("raffle_sc_address")]
    fn raffle_sc_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getTreasuryAddress)]
    #[storage_mapper("treasury_address")]
    fn treasury_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getRaffleScFee)]
    #[storage_mapper("raffle_sc_fee")]
    fn raffle_sc_fee(&self) -> SingleValueMapper<u64>;

    #[view(getTreausryFee)]
    #[storage_mapper("treasury_fee")]
    fn treasury_fee(&self) -> SingleValueMapper<u64>;

    //////////////////////////////////////////////////////////////////////////
    
    #[view(getAvailableTokens)]
    #[storage_mapper("available_tokens")]
    fn available_tokens(&self) -> UnorderedSetMapper<TokenIdentifier>;
    
    //////////////////////////////////////////////////////////////////////////
    
    #[view(getOffer)]
    #[storage_mapper("offers")]
    fn offers(&self, offer_id: usize) -> SingleValueMapper<Offer<Self::Api>>;

    #[view(getOfferIds)]
    #[storage_mapper("offer_ids")]
    fn offer_ids(&self) -> UnorderedSetMapper<usize>;

    #[view(getLastOfferId)]
    #[storage_mapper("last_offer_id")]
    fn last_offer_id(&self) -> SingleValueMapper<usize>;

    #[view(getOfferIdsByFromToken)]
    #[storage_mapper("offer_ids_by_from_token")]
    fn offer_ids_by_from_token(&self, from_token: &TokenIdentifier) -> UnorderedSetMapper<usize>;

    #[view(getOfferIdsByToToken)]
    #[storage_mapper("offer_ids_by_to_token")]
    fn offer_ids_by_to_token(&self, from_token: &TokenIdentifier) -> UnorderedSetMapper<usize>;
}