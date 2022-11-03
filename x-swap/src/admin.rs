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
    ) {
        self.raffle_sc_address().set(&raffle_sc_address);
        self.treasury_address().set(&treasury_address);
        self.raffle_sc_fee().set(raffle_sc_fee);
        self.treasury_fee().set(treasury_fee);
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
    #[endpoint(addAvailableTokens)]
    fn add_available_tokens(
        &self,
        tokens: MultiValueEncoded<TokenIdentifier>,
    ) {
        let mut available_tokens = self.available_tokens();
        for token in tokens {
            available_tokens.insert(token);
        }
    }

    #[only_owner]
    #[endpoint(removeAvailableTokens)]
    fn remove_available_tokens(
        &self,
        tokens: MultiValueEncoded<TokenIdentifier>,
    ) {
        let mut available_tokens = self.available_tokens();
        for token in tokens {
            available_tokens.swap_remove(&token);
        }
    }
}