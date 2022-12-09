elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::data::{ RoundStatus, Round, UserRound, RoundUsersStats };

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

    #[view(getUserRounds)]
    fn get_user_rounds(
        &self,
        address: ManagedAddress,
        start_round_id: usize,
        end_round_id: usize,
    ) -> ManagedVec<UserRound<Self::Api>> {
        let current_round_id = self.current_round_id().get();

        require!(
            start_round_id != 0,
            "start_round_id cannot be zero"
        );
        require!(
            start_round_id <= end_round_id,
            "start_round_id cannot be bigger than end_round_id"
        );
        require!(
            end_round_id <= current_round_id,
            "end_round_id cannot be bigger than current_round_id"
        );

        let mut user_rounds = ManagedVec::new();
        for round_id in start_round_id..=end_round_id {
            let mut ticket_numbers: ManagedVec<usize> = ManagedVec::new();
            let rutn_vec = self.round_user_ticket_numbers(round_id, &address);
            for tn in rutn_vec.iter() {
                ticket_numbers.push(tn);
            }

            let mut win_ticket_number: usize = 0;
            let rwn_vec = self.round_win_numbers(round_id);
            for rwn in rwn_vec.iter() {
                if address == self.ticket_owner(rwn).get() {
                    win_ticket_number = rwn;
                    break;  // there is only one prize for a User per round
                }
            }

            user_rounds.push(
                UserRound {
                ticket_numbers,
                win_ticket_number,
                prize_ranking: self.ticket_prize_ranking(win_ticket_number).get(),
                prize_claimed: self.ticket_claimed(win_ticket_number).get(),
            })
        }

        user_rounds
    }

    #[view(getRoundUsersStats)]
    fn get_round_users_stats(&self, round_id: usize) -> RoundUsersStats<Self::Api> {
        let mut round_users: ManagedVec<ManagedAddress> = ManagedVec::new();
        let mut round_user_ticket_numbers: ManagedVec<ManagedVec<usize>> = ManagedVec::new();

        for ru in self.round_users(round_id).iter() {
            let mut utn_vec = ManagedVec::new();
            for tn in self.round_user_ticket_numbers(round_id, &ru).iter() {
                utn_vec.push(tn);
            }
            round_user_ticket_numbers.push(utn_vec);
            round_users.push(ru);
        }

        RoundUsersStats {
            round_id,
            round_users,
            round_user_ticket_numbers,
        }
    }
}