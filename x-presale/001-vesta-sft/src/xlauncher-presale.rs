#![no_std]


use xlauncher_staking::ProxyTrait as _;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const EGLD_DECIMALS_VALUE: u64 = 1_000_000_000_000_000_000;
const ZERO: u64 = 0;

#[elrond_wasm::derive::contract]
pub trait XLauncherPresale {
    #[init]
    fn init(
        &self,
        token_id: TokenIdentifier,
        initial_price: BigUint,
        min_amount: BigUint,
        max_amount_val: BigUint,
        max_balance_val: BigUint,
        staking_address_val: ManagedAddress,
    ) {
        require!(
            token_id.is_valid_esdt_identifier(),
            "Invalid token identifier"
        );
        require!(initial_price > 0, "Initial price must be positive value");
        require!(min_amount > 0, "Min amount must be positive value");
        require!(max_amount_val > 0, "Max amount must be positive value");
        require!(
            min_amount < max_amount_val,
            "min amount must be smaller then max amount"
        );
        require!(max_balance_val > 0, "Max balance must be positive value");
        self.token_id().set(&token_id);
        self.price().set(initial_price);
        self.min_amount().set(min_amount);
        self.max_amount().set(max_amount_val);
        self.max_balance().set(max_balance_val);
        self.staking_address().set(staking_address_val);
    }

    // I recommend to format the code before each commit (if you use VSCode) 
    // To be easier to read and most importantly for comparing future commits

    // NOTE - for consistency, try to use this particular order:
    // #[only_owner]
    // #[payable("*")]
    // #[endpoint(fundContract)]

    // NOTE
    // You only check for the token identifier, so there is no need to use annotations, as they are not recommended anymore
    // You can use the Call Value API for the token identifier only -> self.call_value().token()
    #[payable("*")]
    #[endpoint(fundContract)]
    fn fund_contract(
        &self,
        #[payment_token] token_identifier: EgldOrEsdtTokenIdentifier,
        #[payment] _payment: BigUint,
    ) {
        require!(!self.token_id().is_empty(), "Token id not set");
        let my_token_id = self.token_id().get();
        require!(my_token_id == token_identifier, "Invalid fund token")
    }

    // NOTE
    // Why rely on the balance of the SC and not save the remaining amount in a storage (also as a view)?
    // If in the future you decide to use those tokens in other places as well, it may be difficult to migrate the logic
    // You could use the self.call_value().payment_token_pair() call in the fund_contract method to check the token_id and to store the amount
    #[view(getTokenBalance)]
    fn get_token_balance(&self) -> BigUint {
        let my_token_id = self.token_id().get();

        let egld_or_esdt_token_identifier = EgldOrEsdtTokenIdentifier::esdt(my_token_id);
        let balance: BigUint = self.blockchain().get_sc_balance(&egld_or_esdt_token_identifier, 0);
        return balance;
    }

    // NOTE
    // From what I see, max_balance() represents the total number of tokens an address can buy
    // If you don't save the amount of tokens already bought by each address, this can be easily exploitable
    // If you check only for the current_balance, the user can buy tokens, send them to another address and buy again the max amount
    // Also, this will not work as intended, as get_esdt_token_data() only works for addresses that are in the same shard as the smart contract
    #[payable("EGLD")]
    #[endpoint]
    fn buy(&self) {
        let egld_or_esdt_token_identifier = self.call_value().egld_or_single_esdt();
        let payment_token = egld_or_esdt_token_identifier.token_identifier;
        let payment_amount = egld_or_esdt_token_identifier.amount;
        require!(self.sell_is_active(), "Sell is not active");
        require!(payment_token.is_egld(), "Only EGLD");
        let min_amount = self.min_amount().get();
        require!(
            payment_amount >= self.min_amount().get(),
            "Payment amount is too low, payment_amount={}, min_amount={}",
            payment_amount, min_amount
        );
        require!(
            payment_amount <= self.max_amount().get(),
            "Payment amount is too high"
        );
        let balance = self.get_token_balance();
        require!(balance > ZERO, "No more tokens to sale.");
        let current_price = self.price().get();
        let one_egld = BigUint::from(EGLD_DECIMALS_VALUE);
        let result_esdt_token_amount = (&current_price * &payment_amount) / &one_egld;

        // Maybe balance >= result_esdt_token_amount to cover all available ESDT amount
        require!(
            balance > result_esdt_token_amount,
            "Not enough tokens for sale."
        );

        let caller = self.blockchain().get_caller();
        //check staking contract
        sc_print!("Hello buy from preslae result_esdt_token_amount={}", result_esdt_token_amount);
        let sc_address = self.staking_address().get();
        let client_total_staked_value: BigUint = self.contract_proxy(sc_address)
            .get_client_total_staked_value(&caller)
            .execute_on_dest_context();

        let x_val = BigUint::from(25_u64);
        let ten_k = BigUint::from(10_000_0_u64);
        let client_max_amount_egld = &client_total_staked_value * &x_val / &ten_k;
        let client_max_balance = (&current_price * &client_max_amount_egld) / &one_egld;
        let mut val_max_balance = self.max_balance().get();
        if client_max_balance <= val_max_balance {
            val_max_balance = client_max_balance.clone();
        }

        sc_print!("Debug info client_total_staked_value={}, x_val={}, ten_k={}, client_max_balance={}, val_max_balance={}, max_amount={}",
            client_total_staked_value, x_val, ten_k, client_max_balance, val_max_balance, self.max_amount().get());

        let token_id_val = self.token_id().get();

        let client_current_balance = self.client_bought_value(&caller).get();
        let client_total_balance = &result_esdt_token_amount + &client_current_balance;
        let _max_balance_val = self.max_balance().get();
        sc_print!("current_value={}",client_current_balance);
        require!(
            client_total_balance <= client_max_balance,
            "Buying that much will exceed max allowed client balance client_total_balance={},  client_max_balance={}",
            client_total_balance, client_max_balance
        );
        self.client_bought_value(&caller).set(client_total_balance);
        sc_print!("Hello there client {}", result_esdt_token_amount.clone());

        self.append_client_if_needed();
        self.send()
            .direct_esdt(&caller, &token_id_val, 0, &result_esdt_token_amount);
    }

    #[payable("*")]
    #[endpoint(buyback)]
    fn buyback(&self) {
        let egld_or_esdt_token_identifier = self.call_value().egld_or_single_esdt();
        let amount = egld_or_esdt_token_identifier.amount;
        let token_id = egld_or_esdt_token_identifier.token_identifier;

        require!(token_id.is_valid(), "invalid token_id");
        let my_token_id = self.token_id().get();
        require!(my_token_id == token_id, "not the same token id");
        require!(BigUint::zero()< amount,"amount needs to be grater then zero");

        let client = self.blockchain().get_caller();
        let presale_balance = self.client_bought_value(&client).get();
        let client_current_buyback_balance = self.client_buyback_value(&client).get();
        sc_print!("client_current_buyback_balance={}",&client_current_buyback_balance);

        let client_total_buyback_balance = &client_current_buyback_balance + &amount;
        require!(
            &client_total_buyback_balance <= &presale_balance,
            "Returning that much will exceed the amount you both during presale: presale-value={}, total-buyback={}",
            presale_balance, client_total_buyback_balance
        );
        self.client_buyback_value(&client).set(client_total_buyback_balance);

        let current_price = self.price().get();
        let one_egld = BigUint::from(EGLD_DECIMALS_VALUE);
        let egld_value = (&amount * &one_egld) / &current_price;

        self.send().direct_egld(&client, &egld_value)
    }

    // NOTE
    // No need for get_balance() method as you can use get_sc_balance for EGLD as well
    // From Docs: For fungible ESDT, nonce should be 0. To get the EGLD balance, you can simply pass TokenIdentifier::egld() as parameter
    // For flexibility, I would recommend 2 different methods for each token (collect_esdt & collect_egld)
    #[only_owner]
    #[endpoint]
    fn collect(&self) {
        let owner = self.blockchain().get_owner_address();

        let sc_address: ManagedAddress = self.blockchain().get_sc_address();
        let egld_balance = self.blockchain().get_balance(&sc_address);

        let my_token_id = self.token_id().get();
        let egld_or_esdt_token_identifier = EgldOrEsdtTokenIdentifier::esdt(my_token_id.clone());
        let token_balance = self.blockchain().get_sc_balance(&egld_or_esdt_token_identifier, 0);

        let big_zero: BigUint = BigUint::zero();
        if big_zero < token_balance {
            self.send()
                .direct_esdt(&owner, &my_token_id, 0, &token_balance);
        }

        if big_zero < egld_balance {
            self.send().direct_egld(&owner, &egld_balance)
        }
    }

    fn append_client_if_needed(&self) {
        let mut clients_set = self.client_list();
        let client = self.blockchain().get_caller();
        if !clients_set.contains(&client) {
            clients_set.insert(client);
        }
    }

    fn sell_is_active(&self) -> bool {
        return if self.sell_active_state().is_empty() {
            false
        } else {
            self.sell_active_state().get()
        };
    }

    #[only_owner]
    #[endpoint(switchSellIsActiveToTrue)]
    fn switch_sell_is_active_to_true(&self) {
        let simple_id = 1_u64;
        sc_print!("hello from switch_sell_is_active_to_true {}", simple_id);
        self.sell_active_state().set(true);
    }

    #[only_owner]
    #[endpoint(switchSellIsActiveToFalse)]
    fn switch_sell_is_active_to_false(&self) {
        let simple_id = 1_u64;
        sc_print!("hello from switch_sell_is_active_to_false {}", simple_id);
        self.sell_active_state().set(false);
    }

    // proxy
    #[proxy]
    fn contract_proxy(&self, sc_address: ManagedAddress) -> xlauncher_staking::Proxy<Self::Api>;

    // storage

    // NOTE
    // For our case this type of storage is simple and good enough
    // But I recommend you to read about FungibleTokenMapper and NonFungibleTokenMapper for future contracts
    #[view(getTokenId)]
    #[storage_mapper("tokenId")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getPrice)]
    #[storage_mapper("price")]
    fn price(&self) -> SingleValueMapper<BigUint>;

    #[view(getMinAmount)]
    #[storage_mapper("minAmount")]
    fn min_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getMaxAmount)]
    #[storage_mapper("maxAmount")]
    fn max_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getMaxBalance)]
    #[storage_mapper("maxBalance")]
    fn max_balance(&self) -> SingleValueMapper<BigUint>;

    #[view(getStakingAddress)]
    #[storage_mapper("stakingAddress")]
    fn staking_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getSellActiveState)]
    #[storage_mapper("sellActiveState")]
    fn sell_active_state(&self) -> SingleValueMapper<bool>;


    #[view(getClientList)]
    #[storage_mapper("clientList")]
    fn client_list(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getClientBoughtValue)]
    #[storage_mapper("clientBoughtValue")]
    fn client_bought_value(
        &self,
        client_address: &ManagedAddress,
    ) -> SingleValueMapper<BigUint>;

    #[view(clientBuybackValue)]
    #[storage_mapper("clientBuybackValue")]
    fn client_buyback_value(
        &self,
        client_address: &ManagedAddress,
    ) -> SingleValueMapper<BigUint>;
}
