elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::data::{ Offer };

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
    #[view(getWegldTokenId)]
    #[storage_mapper("wegld_token_id")]
    fn wegld_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    // When EGLD is traded by an offer, both of maker and taker will receive XRF token equal to the floor of EGLD_amount / this_value.
    #[view(getEgldAmountForIncentive)]
    #[storage_mapper("egld_base_amount_for_incentive")]
    fn egld_base_amount_for_incentive(&self) -> SingleValueMapper<BigUint>;

    #[view(getIncentiveTokenId)]
    #[storage_mapper("incentive_token_id")]
    fn incentive_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getIncentiveBaseAmount)]
    #[storage_mapper("incentive_base_amount")]
    fn incentive_base_amount(&self) -> SingleValueMapper<BigUint>;

    //////////////////////////////////////////////////////////////////////////
    // Token Pairs - One can only create A->B or B->A pairs

    #[view(getATokens)]
    #[storage_mapper("a_tokens")]
    fn a_tokens(&self) -> UnorderedSetMapper<TokenIdentifier>;

    #[view(getBTokens)]
    #[storage_mapper("b_tokens")]
    fn b_tokens(&self) -> UnorderedSetMapper<TokenIdentifier>;
    
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