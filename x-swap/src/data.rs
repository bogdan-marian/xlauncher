elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, ManagedVecItem, Clone)]
pub struct Offer<M: ManagedTypeApi>  {
    pub from: ManagedAddress<M>,
    pub from_token: TokenIdentifier<M>,
    pub from_amount: BigUint<M>,
    pub from_initial_amount: BigUint<M>,

    pub from_timestamp: u64,

    pub to_token: TokenIdentifier<M>,
    pub to_amount: BigUint<M>,
    pub to_initial_amount: BigUint<M>,

    //todo: remove this field. We accept any type of partial acceptance
    pub min_to_amount_per_accept: BigUint<M>,
}