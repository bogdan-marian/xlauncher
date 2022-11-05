elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::config::{ TOTAL_PERCENTAGE };
use crate::state::{ RoundStatus };

#[elrond_wasm::module]
pub trait UserModule:
    crate::storage::StorageModule
    + crate::event::EventModule
    + crate::admin::AdminModule
{
    #[payable("*")]
    #[endpoint(buyTickets)]
    fn buy_tickets(
        &self,
        number_of_tickets: u64,
    ) {
        let round_id = self.current_round_id().get();
        let round_status = self.get_round_status(round_id);
        require!(
            round_status != RoundStatus::Pending,
            "Round is not opened yet."
        );
        require!(
            !(round_status == RoundStatus::Closed || round_status == RoundStatus::Claimable),
            "Round is closed."
        );

        let payment = self.call_value().single_esdt();
        require!(
            payment.token_identifier == self.ticket_token().get(),
            "Wrong payment token."
        );
        require!(
            payment.amount >= self.round_ticket_price(round_id).get() * number_of_tickets,
            "Not enough payment."
        );

        let caller = self.blockchain().get_caller();
        let mut round_user_tickets = self.round_user_tickets(round_id);
        let new_number_of_tickets = round_user_tickets.get(&caller).unwrap_or_default() + number_of_tickets as usize;
        round_user_tickets.insert(caller, new_number_of_tickets);
    }

    #[endpoint(claimPrize)]
    fn claim_prize(
        &self,
        round_id: usize,
    ) {
        let round_status = self.get_round_status(round_id);
        require!(
            round_status == RoundStatus::Claimable,
            "Round is not closed or Winners are not decided yet."
        );

        let caller = self.blockchain().get_caller();
        let user_ranking = self.round_user_ranking(round_id, &caller).get();
        require!(
            user_ranking > 0,
            "You have not won any prize."
        );

        let prize_percentage = self.round_prize_percentages(round_id).get(user_ranking);

        let round_prize_tokens = self.round_prize_tokens(round_id);
        let mut round_left_tokens = self.round_left_tokens(round_id);
        let mut payments = ManagedVec::new();
        for (token_identifier, amount) in round_prize_tokens.iter() {
            let pa = amount * prize_percentage / TOTAL_PERCENTAGE;
            
            round_left_tokens.insert(token_identifier.clone(), round_left_tokens.get(&token_identifier).unwrap_or_default() - &pa);

            payments.push(EsdtTokenPayment::new(token_identifier, 0, pa));
        }

        // send prize to the caller
        self.send().direct_multi(&caller, &payments);
    }
}