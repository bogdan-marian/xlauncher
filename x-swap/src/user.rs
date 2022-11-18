elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::config::{ TOTAL_PERCENTAGE };
use crate::data::{ Offer };

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
        min_to_amount_per_accept_opt: OptionalValue<BigUint>,
    ) {
        let from_payment = self.call_value().single_esdt();

        require!(
            self.a_tokens().contains(&from_payment.token_identifier) && self.b_tokens().contains(&to_token) || self.a_tokens().contains(&to_token) && self.b_tokens().contains(&from_payment.token_identifier),
            "Offering tokens and Expecting tokens are not approved."
        );
        require!(
            from_payment.token_identifier != to_token,
            "Offering token and expecting token must be different."
        );
        // set min_to_amount_per_accept as to_amount if min_to_amount_per_accept_opt is not given
        let min_to_amount_per_accept = match min_to_amount_per_accept_opt {
            OptionalValue::Some(v) => v,
            OptionalValue::None => to_amount.clone(),
        };

        let caller = self.blockchain().get_caller();
        let from_timestamp = self.blockchain().get_block_timestamp();
        let offer_id = self.last_offer_id().get() + 1;
        self.last_offer_id().set(offer_id);

        let offer = Offer {
            from: caller.clone(),
            from_token: from_payment.token_identifier.clone(),
            from_amount: from_payment.amount.clone(),
            from_initial_amount: from_payment.amount.clone(),
            from_timestamp,
            to_token: to_token.clone(),
            to_amount: to_amount.clone(),
            to_initial_amount: to_amount.clone(),
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

        let actual_to_amount = to_payment.amount.clone();
        let actual_from_amount = offer.from_amount.clone() * &to_payment.amount / &offer.to_amount;
        require!(
            actual_to_amount >= offer.min_to_amount_per_accept,
            "Offering token amount must be bigger than the minimum accept amount."
        );

        let caller = self.blockchain().get_caller();
        offer.to_amount -= &actual_to_amount;

        // send incentives to taker
        if offer.from_token == self.wegld_token_id().get() {
            self.send_incentive(&caller, &actual_from_amount);
        } else if offer.to_token == self.wegld_token_id().get() {
            self.send_incentive(&caller, &actual_to_amount);
        }

        // Offer is filled
        if offer.to_amount == BigUint::zero() {
            self.offers(offer_id).clear();
            self.offer_ids().swap_remove(&offer_id);
            self.offer_ids_by_from_token(&offer.from_token).swap_remove(&offer_id);
            self.offer_ids_by_from_token(&offer.to_token).swap_remove(&offer_id);

            // send incentives to maker
            if offer.from_token == self.wegld_token_id().get() {
                self.send_incentive(&offer.from, &offer.from_initial_amount);
            } else if offer.to_token == self.wegld_token_id().get() {
                self.send_incentive(&offer.from, &offer.to_initial_amount);
            }
        // Partial Accept
        } else {
            require!(
                offer.to_amount >= offer.min_to_amount_per_accept,
                "Left offering token amount must be bigger than the minimum accept amount."
            );

            offer.from_amount -= &actual_from_amount;
            self.offers(offer_id).set(&offer);
        }

        // take fee and send it to Raffle Sc and Treasury Wallet
        let from_amount_without_fee = self.take_fee(&offer.from_token, &actual_from_amount);
        let to_amount_without_fee = self.take_fee(&offer.to_token, &actual_to_amount);

        // to Offerer
        self.send().direct_esdt(
            &offer.from,
            &offer.to_token,
            0,
            &to_amount_without_fee,
        );
        // to Acceptor
        self.send().direct_esdt(
            &caller,
            &offer.from_token,
            0,
            &from_amount_without_fee,
        );

        //
        self.accept_offer_event(
            &offer.from,
            &offer.from_token,
            &actual_from_amount,
            &caller,
            &offer.to_token,
            &actual_to_amount,
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

    #[inline]
    fn send_incentive(
        &self,
        to: &ManagedAddress,
        egld_amount: &BigUint,
    ) {
        let incentive_amount = egld_amount.clone() / &self.egld_base_amount_for_incentive().get() * &self.incentive_base_amount().get();
        
        if incentive_amount != BigUint::zero() {
            self.send().direct_esdt(
                to,
                &self.incentive_token_id().get(),
                0,
                &incentive_amount,
            );
        }
    }
}