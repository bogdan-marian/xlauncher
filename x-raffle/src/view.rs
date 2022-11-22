elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::data::{ RoundStatus, Round };

#[elrond_wasm::module]
pub trait ViewModule:
    crate::storage::StorageModule
{
    #[inline]
    fn get_round_status(
        &self,
        round_id: usize,
    ) -> RoundStatus {
        self.round_status(round_id).get()
    }

    /// 1 - Pending
    /// 2 - Opened
    /// 3 - Closed
    #[view(getCurrentRoundStatus)]
    fn get_current_round_status(&self) -> usize {
        let round_id = self.current_round_id().get();
        let round_status = self.get_round_status(round_id);
        
        match round_status {
            RoundStatus::Opened => 2,
            RoundStatus::Closed => 3,
            _ => 1,
        }
    }

    #[view(getRound)]
    fn get_round(&self, round_id: usize) -> Round<Self::Api> {
        let round_status = self.get_round_status(round_id);

        let mut round_prize_tokens = ManagedVec::new();
        let rpt_map = self.round_prize_tokens(round_id);
        for (token_identifier, amount) in rpt_map.iter() {
            round_prize_tokens.push(EsdtTokenPayment::new(token_identifier, 0, amount));
        }
        let mut round_left_tokens = ManagedVec::new();
        let rlt_map = self.round_left_tokens(round_id);
        for (token_identifier, amount) in rlt_map.iter() {
            round_left_tokens.push(EsdtTokenPayment::new(token_identifier, 0, amount));
        }
        
        let mut round_prize_percentages = ManagedVec::new();
        let rpp_vec = self.round_prize_percentages(round_id);
        for rpp in rpp_vec.iter() {
            round_prize_percentages.push(rpp);
        }

        let mut round_winners = ManagedVec::new();
        let rw_vec = self.round_winners(round_id);
        for rw in rw_vec.iter() {
            round_winners.push(rw);
        }

        let mut round_sold_tickets = 0;
        let rut_map = self.round_user_tickets(round_id);
        for (_address, amount) in rut_map.iter() {
            round_sold_tickets += amount;
        }
        let round_sold_amount = self.round_ticket_price(round_id).get() * &BigUint::from(round_sold_tickets);
        
        Round {
            round_id,
            round_status,
            round_start_timestamp: self.round_start_timestamp(round_id).get(),
            round_end_timestamp: self.round_end_timestamp(round_id).get(),
            round_ticket_token: self.ticket_token().get(),
            round_ticket_price: self.round_ticket_price(round_id).get(),
        
            round_prize_tokens,
            round_left_tokens,
        
            round_number_of_winners: self.round_number_of_winners(round_id).get(),
            round_prize_percentages,
            round_winners,
        
            round_sold_tickets,
            round_sold_amount,
        }
    }
}