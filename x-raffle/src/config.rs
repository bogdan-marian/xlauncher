elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::constant::TOTAL_PERCENTAGE;

#[elrond_wasm::module]
pub trait ConfigModule:
    crate::storage::StorageModule
    + crate::event::EventModule
{
    #[only_owner]
    #[endpoint(setSettings)]
    fn set_settings(
        &self,
        treasury_address: ManagedAddress,
        ticket_token: TokenIdentifier,
        ticket_price: BigUint,
        number_of_winners: usize,
        pps: MultiValueEncoded<u64>,
    ) {
        self.set_treasury_address(treasury_address);
        self.set_ticket_token(ticket_token);
        self.set_ticket_price(ticket_price);
        self.set_prize_settings(number_of_winners, pps);
    }

    #[only_owner]
    #[endpoint(setTicketToken)]
    fn set_ticket_token(
        &self,
        ticket_token: TokenIdentifier,
    ) {
        self.ticket_token().set(&ticket_token);
    }

    #[only_owner]
    #[endpoint(setTicketPrice)]
    fn set_ticket_price(
        &self,
        ticket_price: BigUint,
    ) {
        self.ticket_price().set(&ticket_price);
    }

    #[only_owner]
    #[endpoint(setPrizeSettings)]
    fn set_prize_settings(
        &self,
        number_of_winners: usize,
        pps: MultiValueEncoded<u64>,
    ) {
        require!(
            number_of_winners == pps.len(),
            "number_of_winners and the length of prize_percentages do not match."
        );

        self.number_of_winners().set(number_of_winners);

        let mut prize_percentages = self.prize_percentages();
        prize_percentages.clear();
        let mut tp = 0;
        for pp in pps.into_iter() {
            prize_percentages.push(&pp);
            tp += pp;
        }

        require!(
            tp == TOTAL_PERCENTAGE,
            "Sum of prize_percentages is not equal to {}",
            TOTAL_PERCENTAGE
        );
    }

    #[only_owner]
    #[endpoint(setTreasuryAddress)]
    fn set_treasury_address(
        &self,
        treasury_address: ManagedAddress,
    ) {
        self.treasury_address().set(&treasury_address);
    }
}