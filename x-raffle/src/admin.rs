elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::data::{ RoundStatus };

#[elrond_wasm::module]
pub trait AdminModule:
    crate::storage::StorageModule
    + crate::event::EventModule
    + crate::view::ViewModule
{
    #[only_owner]
    #[endpoint(openGenesisRound)]
    fn open_genesis_round(&self) {
        let mut round_id = self.current_round_id().get();
        require!(
            round_id == 0,
            "Genesis Round was already started."
        );

        round_id += 1;

        self.last_ticker_number().set(0); // this is an emphasis (initial value of usize SingleValueMapper is 0)
        self.round_first_ticket_number(round_id).set(1);
        self.round_first_ticket_number(round_id + 1).set(1);

        self.current_round_id().set(round_id);
        self.round_status(round_id).set(RoundStatus::Opened);

        let current_timestamp = self.blockchain().get_block_timestamp();
        self.round_start_timestamp(round_id).set(current_timestamp);

        //
        self.round_ticket_price(round_id).set(&self.ticket_price().get());
        self.round_number_of_winners(round_id).set(self.number_of_winners().get());
    }

    #[inline]
    fn open_round(&self, round_id: usize) {
        require!(
            self.round_status(round_id).get() == RoundStatus::Pending,
            "Given round is already started."
        );

        self.current_round_id().set(round_id);
        self.round_status(round_id).set(RoundStatus::Opened);

        let current_timestamp = self.blockchain().get_block_timestamp();
        self.round_start_timestamp(round_id).set(current_timestamp);

        //
        self.round_ticket_price(round_id).set(&self.ticket_price().get());
        self.round_number_of_winners(round_id).set(self.number_of_winners().get());

        let mut round_prize_percentages = self.round_prize_percentages(round_id);
        let prize_percentages = self.prize_percentages();
        for p in prize_percentages.iter() {
            round_prize_percentages.push(&p);
        }

        self.round_first_ticket_number(round_id + 1).set(self.round_first_ticket_number(round_id).get());
    }

    #[payable("*")]
    #[endpoint(injectPrize)]
    fn inject_prize(&self) {
        let mut round_id = self.current_round_id().get();
        let round_status = self.get_round_status(round_id);
        require!(
            round_id > 0,
            "No Round is opened yet."
        );

        // if current round is closed, rewards will be injected into the next round
        if round_status == RoundStatus::Closed {
            round_id += 1;
        }

        // update round_prize_tokens
        let payments = self.call_value().all_esdt_transfers();
        let mut round_prize_tokens = self.round_prize_tokens(round_id);
        for payment in payments.iter() {
            let amount = round_prize_tokens.get(&payment.token_identifier).unwrap_or_default();
            round_prize_tokens.insert(payment.token_identifier, amount + &payment.amount);
        }
    }

    #[only_owner]
    #[endpoint(finishAndStartNewRound)]
    fn finish_and_start_new_round(&self) {
        //TODO: check that shis is not the current active round
        let round_id = self.current_round_id().get();
        self.finish_round(round_id);
        self.open_round(round_id + 1);
    }

    #[inline]
    fn finish_round(&self, round_id: usize) {
        let round_status = self.get_round_status(round_id);
        require!(
            round_id > 0,
            "No Round is opened yet."
        );
        require!(
            round_status != RoundStatus::Pending,
            "Current Round is not opened yet."
        );
        self.round_status(round_id).set(RoundStatus::Closed);

        let win_ticket_numbers = self.choose_winners();
        let mut ranking = 1;
        for wtn in win_ticket_numbers.iter() {
            self.ticket_prize_ranking(wtn).set(ranking);
            self.round_win_numbers(round_id).push(&wtn);
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
    fn choose_winners(&self) -> ManagedVec<usize> {
        //TODO: check that this is not the current active lottery round
        let round_id = self.current_round_id().get();
        let number_of_winners = self.round_number_of_winners(round_id).get();

        let current_round_first_ticket_number = self.round_first_ticket_number(round_id).get();
        let next_round_first_ticket_number = self.round_first_ticket_number(round_id + 1).get();
        
        let mut ticket_numbers_vec: ManagedVec<usize> = ManagedVec::new();
        for tn in current_round_first_ticket_number..next_round_first_ticket_number {
            ticket_numbers_vec.push(tn);
        }

        let mut rand = RandomnessSource::<Self::Api>::new();
        let mut win_ticket_numbers: ManagedVec<usize> = ManagedVec::new();
        for _ in 0..number_of_winners {
            if ticket_numbers_vec.len() == 0 { // if no ticket is left, break the loop
                break;
            }

            let rand_index = rand.next_usize_in_range(0, ticket_numbers_vec.len());

            //
            win_ticket_numbers.push(ticket_numbers_vec.get(rand_index));

            //
            let winner = self.ticket_owner(ticket_numbers_vec.get(rand_index)).get();
            let mut new_win_ticket_numbers: ManagedVec<usize> = ManagedVec::new();
            for i in ticket_numbers_vec.iter() {
                if winner != self.ticket_owner(ticket_numbers_vec.get(i)).get() {
                    new_win_ticket_numbers.push(ticket_numbers_vec.get(i));
                }
            }
            ticket_numbers_vec = new_win_ticket_numbers;
        }

        win_ticket_numbers
    }
}