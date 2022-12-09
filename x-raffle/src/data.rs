elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq)]
pub enum RoundStatus {
    Pending,
    Opened,
    Closed,
}

// this struct is too costly, do not use this in non-view functions
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq)]
pub struct Round<M: ManagedTypeApi> {
    pub round_id: usize,
    pub round_status: RoundStatus,
    pub round_start_timestamp: u64,
    pub round_end_timestamp: u64,
    pub round_ticket_token: TokenIdentifier<M>,
    pub round_ticket_price: BigUint<M>,

    pub round_prize_tokens: ManagedVec<M, EsdtTokenPayment<M>>,
    pub round_left_tokens: ManagedVec<M, EsdtTokenPayment<M>>,

    pub round_number_of_winners: usize,
    pub round_prize_percentages: ManagedVec<M, u64>,
    pub round_winners: ManagedVec<M, ManagedAddress<M>>,
    pub round_win_numbers: ManagedVec<M, usize>,

    pub round_sold_tickets: usize,
    pub round_sold_amount: BigUint<M>,
}

// this struct is too costly, do not use this in non-view functions
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq)]
pub struct User<M: ManagedTypeApi> {
    pub address: ManagedAddress<M>,
    pub round_ticket_numbers: ManagedVec<M, ManagedVec<M, usize>>,
    pub round_prize_rankings: ManagedVec<M, usize>,
    pub round_prize_claimed: ManagedVec<M, bool>,
}

// this struct is too costly, do not use this in non-view functions
#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq)]
pub struct RoundUsersStats<M: ManagedTypeApi> {
    pub round_id: usize,
    pub round_users: ManagedVec<M, ManagedAddress<M>>,
    pub round_user_ticket_numbers: ManagedVec<M, ManagedVec<M, usize>>,
}