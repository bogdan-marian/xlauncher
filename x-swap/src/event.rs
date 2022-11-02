elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait EventModule
{
    #[event("MakeOffer")]
    fn make_offer_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] from_token: &TokenIdentifier,
        #[indexed] from_amount: &BigUint,
        #[indexed] from_timestamp: u64,
        #[indexed] to_token: &TokenIdentifier,
        #[indexed] to_amount: &BigUint,
        #[indexed] min_to_amount_per_accept: &BigUint,
    );

    #[event("MakeOffer")]
    fn accept_offer_event(
        &self,
        #[indexed] from: &ManagedAddress,
        #[indexed] from_token: &TokenIdentifier,
        #[indexed] from_amount: &BigUint,
        #[indexed] to: &ManagedAddress,
        #[indexed] to_token: &TokenIdentifier,
        #[indexed] to_amount: &BigUint,
    );
}