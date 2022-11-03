elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::config::{ TOTAL_PERCENTAGE };
use crate::state::{ Offer };

#[elrond_wasm::module]
pub trait UserModule:
    crate::storage::StorageModule
    + crate::event::EventModule
{    
    #[payable("*")]
    #[endpoint(makeOffer)]
    fn make_offer(
        &self,
        to_token: TokenIdentifier,
        to_amount: BigUint,
        min_to_amount_per_accept: BigUint,
    ) {
        let from_payment = self.call_value().single_esdt();

        require!(
            self.available_tokens().contains(&from_payment.token_identifier),
            "Offering token is not allowed."
        );
        require!(
            self.available_tokens().contains(&to_token),
            "Expecting token is not allowed."
        );
        require!(
            from_payment.token_identifier != to_token,
            "Offering token and expecting token must be different."
        );

        let caller = self.blockchain().get_caller();
        let from_timestamp = self.blockchain().get_block_timestamp();
        let offer_id = self.last_offer_id().get() + 1;
        self.last_offer_id().set(offer_id);

        let offer = Offer {
            from: caller.clone(),
            from_token: from_payment.token_identifier.clone(),
            from_amount: from_payment.amount.clone(),
            from_timestamp,
            to_token: to_token.clone(),
            to_amount: to_amount.clone(),
            min_to_amount_per_accept: min_to_amount_per_accept.clone(),
        };

        //
        self.offers(offer_id).set(offer);
        self.offer_ids().insert(offer_id);
        self.offer_ids_by_from_token(&from_payment.token_identifier).insert(offer_id);
        self.offer_ids_by_from_token(&to_token).insert(offer_id);

        //
        self.make_offer_event(
            &caller,
            &from_payment.token_identifier,
            &from_payment.amount,
            from_timestamp,
            &to_token,
            &to_amount,
            &min_to_amount_per_accept,
        );
    }

    #[payable("*")]
    #[endpoint(acceptOffer)]
    fn accept_offer(
        &self,
        offer_id: usize,
        from_percentage: u64,   // how much percentage of offering amount will be accepted
    ) {
        require!(
            self.offer_ids().contains(&offer_id),
            "offer_id does not exist."
        );

        let to_payment = self.call_value().single_esdt();
        let mut offer = self.offers(offer_id).get();

        require!(
            offer.to_token == to_payment.token_identifier,
            "Wrong expecting token."
        );

        let to_amount = offer.to_amount.clone() * from_percentage / TOTAL_PERCENTAGE;
        let from_amount = offer.from_amount.clone() * from_percentage / TOTAL_PERCENTAGE;
        require!(
            to_payment.amount >= to_amount,
            "Offering token is not enough."
        );
        require!(
            to_amount >= offer.min_to_amount_per_accept,
            "Offering token amount must be bigger than the minimum accept amount."
        );

        let caller = self.blockchain().get_caller();

        // Full Accept
        if from_percentage == TOTAL_PERCENTAGE {
            self.offers(offer_id).clear();
            self.offer_ids().swap_remove(&offer_id);
            self.offer_ids_by_from_token(&offer.from_token).swap_remove(&offer_id);
            self.offer_ids_by_from_token(&offer.to_token).swap_remove(&offer_id);

        // Partial Accept
        } else {
            offer.from_amount -= &from_amount;
            offer.to_amount -= &to_amount;
            self.offers(offer_id).set(&offer);

            require!(
                offer.to_amount >= offer.min_to_amount_per_accept,
                "Left offering token amount must be bigger than the minimum accept amount."
            );
        }

        // take fee and send it to Raffle Sc and Treasury Wallet
        let left_from_amount = self.take_fee(&offer.from_token, &from_amount);
        let left_to_amount = self.take_fee(&offer.to_token, &to_amount);

        // to Offerer
        self.send().direct_esdt(
            &offer.from,
            &offer.to_token,
            0,
            &left_to_amount,
        );
        // to Acceptor
        self.send().direct_esdt(
            &caller,
            &offer.from_token,
            0,
            &left_from_amount,
        );

        //
        self.accept_offer_event(
            &offer.from,
            &offer.from_token,
            &from_amount,
            &caller,
            &offer.to_token,
            &to_amount,
        );
    }

    #[inline]
    fn take_fee(
        &self,
        token: &TokenIdentifier,
        amount: &BigUint,
    ) -> BigUint {
        let raffle_sc_fee = self.raffle_sc_fee().get();
        let treasury_fee = self.treasury_fee().get();

        let raffle_sc_amount = amount.clone() * raffle_sc_fee / TOTAL_PERCENTAGE;
        let treasury_amount = amount.clone() * treasury_fee / TOTAL_PERCENTAGE;
        let left_amount = amount.clone() - &raffle_sc_amount - &treasury_amount;

        self.send().direct_esdt(
            &self.raffle_sc_address().get(),
            &token,
            0,
            &raffle_sc_amount,
        );
        self.send().direct_esdt(
            &self.treasury_address().get(),
            &token,
            0,
            &treasury_amount,
        );

        left_amount
    }
}