elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::data::{ RoundStatus };

#[elrond_wasm::module]
pub trait StorageModule
{
    #[view(getTicketToken)]
    #[storage_mapper("ticket_token")]
    fn ticket_token(&self) -> SingleValueMapper<TokenIdentifier>;

    //////////////////////////////////////////////////////////////////////////
        
    #[view(getCurrentRoundId)]
    #[storage_mapper("current_round_id")]
    fn current_round_id(&self) -> SingleValueMapper<usize>;

    #[view(getRoundStatus)]
    #[storage_mapper("round_status")]
    fn round_status(&self, round_id: usize) -> SingleValueMapper<RoundStatus>;

    #[view(getRoundStartTimestamp)]
    #[storage_mapper("round_start_timestamp")]
    fn round_start_timestamp(&self, round_id: usize) -> SingleValueMapper<u64>;

    #[view(getRoundEndTimestamp)]
    #[storage_mapper("round_end_timestamp")]
    fn round_end_timestamp(&self, round_id: usize) -> SingleValueMapper<u64>;

    #[view(getRoundTicketPrice)]
    #[storage_mapper("round_ticket_price")]
    fn round_ticket_price(&self, round_id: usize) -> SingleValueMapper<BigUint>;

    //
    #[view(getRoundPrizeTokens)]
    #[storage_mapper("round_prize_tokens")]
    fn round_prize_tokens(&self, round_id: usize) -> MapMapper<TokenIdentifier, BigUint>;

    #[view(getRoundLeftTokens)]
    #[storage_mapper("round_left_tokens")]
    fn round_left_tokens(&self, round_id: usize) -> MapMapper<TokenIdentifier, BigUint>;

    //
    #[view(getRoundNumberOfWinners)]
    #[storage_mapper("round_number_of_winners")]
    fn round_number_of_winners(&self, round_id: usize) -> SingleValueMapper<usize>;

    #[view(getRoundPrizePercentages)]
    #[storage_mapper("round_prize_percentages")]
    fn round_prize_percentages(&self, round_id: usize) -> VecMapper<u64>;

    #[view(getRoundWinners)]
    #[storage_mapper("round_winners")]
    fn round_winners(&self, round_id: usize) -> VecMapper<ManagedAddress>;

    //
    #[view(getRoundUserTickets)]
    #[storage_mapper("round_user_tickets")]
    fn round_user_tickets(&self, round_id: usize) -> MapMapper<ManagedAddress, usize>;

    #[view(getRoundUserRanking)]
    #[storage_mapper("round_user_ranking")]
    fn round_user_ranking(&self, round_id: usize, user: &ManagedAddress) -> SingleValueMapper<usize>;
}