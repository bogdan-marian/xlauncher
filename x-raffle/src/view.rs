elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::data::{ RoundStatus, Round, User };

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

        let mut round_win_numbers = ManagedVec::new();
        let mut round_winners = ManagedVec::new();
        let rwn_vec = self.round_win_numbers(round_id);
        for rwn in rwn_vec.iter() {
            round_win_numbers.push(rwn);
            round_winners.push(self.ticket_owner(rwn).get());
        }

        let round_sold_tickets = self.round_first_ticket_number(round_id + 1).get () - self.round_first_ticket_number(round_id).get();
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
            round_win_numbers,
        
            round_sold_tickets,
            round_sold_amount,
        }
    }

    #[view(getUser)]
    fn get_user(&self, address: ManagedAddress) -> User<Self::Api> {
        let mut round_ticket_numbers = ManagedVec::new();
        let mut round_prize_rankings = ManagedVec::new();
        let mut round_prize_claimed = ManagedVec::new();

        let current_round_id = self.current_round_id().get();
        for round_id in 1..current_round_id {
            let mut ticket_numbers: ManagedVec<usize> = ManagedVec::new();
            let rutn_vec = self.round_user_ticket_numbers(round_id, &address);
            for tn in rutn_vec.iter() {
                ticket_numbers.push(tn);
            }

            round_ticket_numbers.push(ticket_numbers);

            let mut win_ticket_number: usize = 0;
            let rwn_vec = self.round_win_numbers(round_id);
            for rwn in rwn_vec.iter() {
                if address == self.ticket_owner(rwn).get() {
                    win_ticket_number = rwn;
                    break;  // there is only one prize for a User per round
                }
            }
            round_prize_rankings.push(self.ticket_prize_ranking(win_ticket_number).get());
            round_prize_claimed.push(self.ticket_claimed(win_ticket_number).get());
        }

        User {
            address,
            round_ticket_numbers,
            round_prize_rankings,
            round_prize_claimed,
        }
    }
}