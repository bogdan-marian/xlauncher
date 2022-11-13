use core::ops::Deref;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::module]
pub trait AdminModule:
    crate::storage::StorageModule
    + crate::event::EventModule
{    
    #[only_owner]
    #[endpoint(setSettings)]
    fn set_settings(
        &self,
        raffle_sc_address: ManagedAddress,
        treasury_address: ManagedAddress,
        raffle_sc_fee: u64,
        treasury_fee: u64,

        wegld_token_id: TokenIdentifier,
        egld_base_amount_for_incentive: BigUint,
        incentive_token_id: TokenIdentifier,
        incentive_base_amount: BigUint,

        a_tokens: ManagedVec<TokenIdentifier>,
        b_tokens: ManagedVec<TokenIdentifier>,
    ) {
        self.raffle_sc_address().set(&raffle_sc_address);
        self.treasury_address().set(&treasury_address);
        self.raffle_sc_fee().set(raffle_sc_fee);
        self.treasury_fee().set(treasury_fee);

        self.wegld_token_id().set(&wegld_token_id);
        self.egld_base_amount_for_incentive().set(&egld_base_amount_for_incentive);
        self.incentive_token_id().set(&incentive_token_id);
        self.incentive_base_amount().set(&incentive_base_amount);

        let mut a_tokens_set = self.a_tokens();
        for token in a_tokens.iter() {
            a_tokens_set.insert(token.deref().clone());
        }
        let mut b_tokens_set = self.a_tokens();
        for token in b_tokens.iter() {
            b_tokens_set.insert(token.deref().clone());
        }
    }

    #[only_owner]
    #[endpoint(setRaffleScAddress)]
    fn set_raffle_sc_address(
        &self,
        raffle_sc_address: ManagedAddress,
    ) {
        self.raffle_sc_address().set(&raffle_sc_address);
    }

    #[only_owner]
    #[endpoint(setTreasuryAddress)]
    fn set_treasury_address(
        &self,
        treasury_address: ManagedAddress,
    ) {
        self.treasury_address().set(&treasury_address);
    }

    #[only_owner]
    #[endpoint(setRaffleScFee)]
    fn set_raffle_sc_fee(
        &self,
        raffle_sc_fee: u64,
    ) {
        self.raffle_sc_fee().set(raffle_sc_fee);
    }

    #[only_owner]
    #[endpoint(setTreasuryFee)]
    fn set_treasury_fee(
        &self,
        treasury_fee: u64,
    ) {
        self.treasury_fee().set(treasury_fee);
    }

    #[only_owner]
    #[endpoint(setWegldTokenId)]
    fn set_wegld_token_id(
        &self,
        wegld_token_id: TokenIdentifier,
    ) {
        self.wegld_token_id().set(&wegld_token_id);
    }

    #[only_owner]
    #[endpoint(setIncentiveTokenId)]
    fn set_incentive_token_id(
        &self,
        incentive_token_id: TokenIdentifier,
    ) {
        self.incentive_token_id().set(&incentive_token_id);
    }

    #[only_owner]
    #[endpoint(setIncentiveBaseAmount)]
    fn set_incentive_base_amount(
        &self,
        incentive_base_amount: BigUint,
    ) {
        self.incentive_base_amount().set(&incentive_base_amount);
    }

    #[only_owner]
    #[endpoint(addATokens)]
    fn add_a_tokens(
        &self,
        tokens: MultiValueEncoded<TokenIdentifier>,
    ) {
        let mut available_tokens = self.a_tokens();
        for token in tokens {
            available_tokens.insert(token);
        }
    }

    #[only_owner]
    #[endpoint(removeATokens)]
    fn remove_a_tokens(
        &self,
        tokens: MultiValueEncoded<TokenIdentifier>,
    ) {
        let mut available_tokens = self.a_tokens();
        for token in tokens {
            available_tokens.swap_remove(&token);
        }
    }

    #[only_owner]
    #[endpoint(addBTokens)]
    fn add_b_tokens(
        &self,
        tokens: MultiValueEncoded<TokenIdentifier>,
    ) {
        let mut available_tokens = self.b_tokens();
        for token in tokens {
            available_tokens.insert(token);
        }
    }

    #[only_owner]
    #[endpoint(removeBTokens)]
    fn remove_b_tokens(
        &self,
        tokens: MultiValueEncoded<TokenIdentifier>,
    ) {
        let mut available_tokens = self.b_tokens();
        for token in tokens {
            available_tokens.swap_remove(&token);
        }
    }
}