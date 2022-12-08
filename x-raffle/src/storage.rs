elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::data::{ RoundStatus };

#[elrond_wasm::module]
pub trait StorageModule
{
    #[view(getTicketToken)]
    #[storage_mapper("ticket_token")]
    fn ticket_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getTicketPrice)]
    #[storage_mapper("ticket_price")]
    fn ticket_price(&self) -> SingleValueMapper<BigUint>;

    #[view(getNumberOfWinners)]
    #[storage_mapper("number_of_winners")]
    fn number_of_winners(&self) -> SingleValueMapper<usize>;

    #[view(getPrizePercentages)]
    #[storage_mapper("prize_percentages")]
    fn prize_percentages(&self) -> VecMapper<u64>;

    #[view(getTreasuryAddress)]
    #[storage_mapper("treasury_address")]
    fn treasury_address(&self) -> SingleValueMapper<ManagedAddress>;

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

    //
    #[view(getRoundTicketPrice)]
    #[storage_mapper("round_ticket_price")]
    fn round_ticket_price(&self, round_id: usize) -> SingleValueMapper<BigUint>;

    #[view(getRoundNumberOfWinners)]
    #[storage_mapper("round_number_of_winners")]
    fn round_number_of_winners(&self, round_id: usize) -> SingleValueMapper<usize>;

    #[view(getRoundPrizePercentages)]
    #[storage_mapper("round_prize_percentages")]
    fn round_prize_percentages(&self, round_id: usize) -> VecMapper<u64>;
    //
    #[view(getRoundPrizeTokens)]
    #[storage_mapper("round_prize_tokens")]
    fn round_prize_tokens(&self, round_id: usize) -> MapMapper<TokenIdentifier, BigUint>;

    #[view(getRoundLeftTokens)]
    #[storage_mapper("round_left_tokens")]
    fn round_left_tokens(&self, round_id: usize) -> MapMapper<TokenIdentifier, BigUint>;

    //
    #[view(getLastTickerNumber)]
    #[storage_mapper("last_ticker_number")]
    fn last_ticker_number(&self) -> SingleValueMapper<usize>;

    #[view(getTicketOwner)]
    #[storage_mapper("ticket_owner")]
    fn ticket_owner(&self, ticket_number: usize) -> SingleValueMapper<ManagedAddress>;

    #[view(getTicketPrizeRanking)]
    #[storage_mapper("ticket_prize_ranking")]
    fn ticket_prize_ranking(&self, ticket_number: usize) -> SingleValueMapper<usize>;   // ranking of ticket in a round; if no rank, value is 0

    #[view(getTicketClaimed)]
    #[storage_mapper("ticket_claimed")]
    fn ticket_claimed(&self, ticket_number: usize) -> SingleValueMapper<bool>;

    //
    #[view(getRoundFirstTicketNumber)]
    #[storage_mapper("round_first_ticket_number")]
    fn round_first_ticket_number(&self, round_id: usize) -> SingleValueMapper<usize>;

    #[view(getRoundWinNumbers)]
    #[storage_mapper("round_win_numbers")]
    fn round_win_numbers(&self, round_id: usize) -> VecMapper<usize>;

    //
    #[view(getRoundUserTicketNumbers)]
    #[storage_mapper("round_user_ticket_numbers")]
    fn round_user_ticket_numbers(&self, round_id: usize, user_address: &ManagedAddress) -> VecMapper<usize>;
}