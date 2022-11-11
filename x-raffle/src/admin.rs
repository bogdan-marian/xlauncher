elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use core::ops::Deref;

use crate::config::{ TOTAL_PERCENTAGE };
use crate::state::{ RoundStatus };

#[elrond_wasm::module]
pub trait AdminModule:
    crate::storage::StorageModule
    + crate::event::EventModule
    + crate::view::ViewModule
{
    #[only_owner]
    #[endpoint(openRound)]
    fn open_round(
        &self,
        end_timestamp: u64,
        ticket_price: BigUint,
        number_of_winners: usize,
        prize_percentages: MultiValueEncoded<u64>,
    ) {
        let mut round_id = self.current_round_id().get();
        require!(
            round_id == 0 || self.get_round_status(round_id) == RoundStatus::Claimable,
            "Previous Round is not closed yet."
        );

        round_id += 1;
        self.current_round_id().set(round_id);
        self.round_status(round_id).set(RoundStatus::Opened);

        let current_timestamp = self.blockchain().get_block_timestamp();
        require!(
            end_timestamp > current_timestamp,
            "end_timestamp cannot be the past."
        );

        self.round_start_timestamp(round_id).set(current_timestamp);
        self.round_end_timestamp(round_id).set(end_timestamp);
        self.round_ticket_price(round_id).set(ticket_price);
        self.round_number_of_winners(round_id).set(number_of_winners);

        require!(
            number_of_winners == prize_percentages.len(),
            "number_of_winners and length of prize_percentages do not match."
        );

        let mut ps = self.round_prize_percentages(round_id);
        let mut tp = 0;
        for p in prize_percentages {
            ps.push(&p);
            tp += p;
        }
        require!(
            tp == TOTAL_PERCENTAGE,
            "Sum of prize_percentages is not equal to {}",
            TOTAL_PERCENTAGE
        );
    }

    #[payable("*")]
    #[endpoint(injectPrize)]
    fn inject_prize(&self) {
        let round_id = self.current_round_id().get();
        let round_status = self.get_round_status(round_id);
        require!(
            round_id > 0,
            "No Round is opened yet."
        );
        // One can only inject prize before Claimable
        require!(
            round_status == RoundStatus::Opened || round_status == RoundStatus::Closed,
            "Current Round is already closed."
        );

        // update round_prize_tokens
        let payments = self.call_value().all_esdt_transfers();
        let mut round_prize_tokens = self.round_prize_tokens(round_id);
        for payment in payments.iter() {
            let amount = round_prize_tokens.get(&payment.token_identifier).unwrap_or_default();
            round_prize_tokens.insert(payment.token_identifier, amount + &payment.amount);
        }
    }

    #[only_owner]
    #[endpoint(finishRound)]
    fn finish_round(&self) {
        //TODO: check that shis is not the current active round
        let round_id = self.current_round_id().get();
        let round_status = self.get_round_status(round_id);
        require!(
            round_id > 0,
            "No Round is opened yet."
        );
        require!(
            round_status != RoundStatus::Claimable,
            "Current Round is already finished."
        );
        require!(
            round_status == RoundStatus::Closed,
            "Current Round is not closed."
        );
        self.round_status(round_id).set(RoundStatus::Claimable);

        let winners = self.choose_winners();
        let mut round_winners = self.round_winners(round_id);
        let mut ranking = 1;
        for winner in winners.iter() {
            let winner_address = winner.deref();
            self.round_user_ranking(round_id, &winner_address).set(ranking);
            round_winners.push(winner_address);
            ranking += 1;
        }

        // update round_left_tokens
        let round_prize_tokens = self.round_prize_tokens(round_id);
        let mut round_left_tokens = self.round_left_tokens(round_id);
        for (token_identifier, amount) in round_prize_tokens.iter() {
            round_left_tokens.insert(token_identifier, amount);
        }
    }

    //
    #[inline]
    fn choose_winners(&self) -> ManagedVec<ManagedAddress> {
        //TODO: check that this is not the current active lottery round
        let round_id = self.current_round_id().get();
        let number_of_winners = self.round_number_of_winners(round_id).get();
        let number_of_users = self.round_user_tickets(round_id).len();

        // if number of people who bought tickets in this round is smaller than number of prizes, real_number_of_prizes will be number of people who bought tickets in this round
        let real_number_of_prizes = core::cmp::min(number_of_winners, number_of_users);

        let mut total_numbers = 0;
        let mut user_tickets_vec: ManagedVec<usize> = ManagedVec::new();
        let mut user_index_vec: ManagedVec<usize> = ManagedVec::new();
        let round_user_tickets = self.round_user_tickets(round_id);
        let mut k = 0;
        for (_, user_number_of_tickets) in round_user_tickets.iter() {
            total_numbers += user_number_of_tickets;
            user_tickets_vec.push(user_number_of_tickets);
            user_index_vec.push(k);
            k += 1;
        }

        let mut rand = RandomnessSource::<Self::Api>::new();
        let mut winner_ids: ManagedVec<usize> = ManagedVec::new();
        for _ in 0..real_number_of_prizes {
            let rand_index = rand.next_usize_in_range(0, total_numbers);

            let mut sum = 0;
            let mut j = 0;
            for num in user_tickets_vec.iter() {
                sum += num;
                if rand_index < sum {
                    break;
                }
                j += 1;
            }

            total_numbers -= user_tickets_vec.get(j);
            winner_ids.push(user_index_vec.get(j));
            user_tickets_vec.remove(j);
            user_index_vec.remove(j);
        }

        let mut winners = ManagedVec::new();
        let mut i = 0;
        let mut j = 0;
        for (user_address, _) in round_user_tickets.iter() {
            if winner_ids.get(i) == j {
                winners.push(user_address);
                i += 1;
            }
            j += 1;
        }

        winners
    }
}