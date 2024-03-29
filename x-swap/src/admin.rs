

elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::module]
pub trait AdminModule:
    crate::storage::StorageModule
    + crate::event::EventModule
{    
    #[payable("*")]
    #[endpoint(injectIncentiveToken)]
    fn inject_incentive_token(&self) {
        let (token_id, _nonce, token_amount) = self.call_value().single_esdt().into_tuple();
        require!(
            token_id == self.incentive_token_id().get(),
            "Token must be incentive_token_id"
        );
        self.incentive_token_amount().update(|v| *v += &token_amount);
    }

    #[only_owner]
    #[endpoint(collectIncentiveToken)]
    fn collect_incentive_token(&self, opt_collect_amount: OptionalValue<BigUint>) {
        let collect_amount = match opt_collect_amount {
            OptionalValue::Some(v) => core::cmp::min(v, self.incentive_token_amount().get()),
            OptionalValue::None => self.incentive_token_amount().get(),
        };

        self.incentive_token_amount().update(|v| *v -= &collect_amount);
        
        self.send().direct_esdt(
            &self.blockchain().get_caller(),
            &self.incentive_token_id().get(),
            0u64,
            &collect_amount,
        );
    }
}