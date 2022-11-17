elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use std::fmt::Debug;

use elrond_wasm::elrond_codec::CodecFrom;
use x_swap::{
    admin::ProxyTrait as _,
    event::ProxyTrait as _,
    storage::ProxyTrait as _,
    user::ProxyTrait as _,
    view::ProxyTrait as _,
    ProxyTrait as _,
};
use elrond_wasm_debug::{
    mandos_system::model::*,
    DebugApi,
    BlockchainMock,
    ContractInfo,
};


const TOTAL_PERCENTAGE: u64 = 100_00;
const WEGLD_ID: &[u8] = b"WEGLD-123456";
const USDC_ID: &[u8] = b"USDC-123456";
const XRF_ID: &[u8] = b"XRF-123456";

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.register_contract_builder(
        "file:output/x-swap.wasm",
        x_swap::ContractBuilder,
    );
    blockchain
}

type XSwapContract = ContractInfo<x_swap::Proxy<DebugApi>>;
struct XSwapTestState<M: ManagedTypeApi>  {
    world: BlockchainMock,
    xswap_contract: XSwapContract,
    owner_address: AddressValue,
    raffle_sc_address: AddressValue,
    treasury_address: AddressValue,
    alice: AddressValue,
    bob: AddressValue,
    raffle_sc_fee: u64,
    treasury_fee: u64,
    wegld_token_id: TokenIdentifier<M>,
    egld_base_amount_for_incentive: BigUint<M>,
    incentive_token_id: TokenIdentifier<M>,
    incentive_base_amount: BigUint<M>,
    a_tokens: ManagedVec<M, TokenIdentifier<M>>,
    b_tokens: ManagedVec<M, TokenIdentifier<M>>,
}

impl XSwapTestState<DebugApi> {
    fn setup() -> Self {
        let world = world();
        let ic = &world.interpreter_context();

        let mut state = XSwapTestState {
            world,
            xswap_contract: XSwapContract::new("sc:x_swap"),
            owner_address: "address:owner".into(),
            raffle_sc_address: "address:raffle_sc".into(),
            treasury_address: "address:treasury".into(),
            alice: "address:alice".into(),
            bob: "address:bob".into(),
            raffle_sc_fee: 500,
            treasury_fee: 500,
            wegld_token_id: TokenIdentifier::from_esdt_bytes(WEGLD_ID),
            egld_base_amount_for_incentive: BigUint::from(5u64),
            incentive_token_id: TokenIdentifier::from_esdt_bytes(XRF_ID),
            incentive_base_amount: BigUint::from(1u64),
            a_tokens: ManagedVec::from_single_item(TokenIdentifier::from_esdt_bytes(WEGLD_ID)),
            b_tokens: ManagedVec::from_single_item(TokenIdentifier::from_esdt_bytes(USDC_ID)),
        };

        state.world.mandos_set_state(
            SetStateStep::new()
                .put_account(&state.owner_address, Account::new().nonce(1))
                .put_account(&state.raffle_sc_address, Account::new().nonce(1))
                .put_account(&state.treasury_address, Account::new().nonce(1))
                .put_account(&state.alice, Account::new().nonce(1))
                .put_account(&state.bob, Account::new().nonce(1))
        );

        state
    }

    fn deploy(&mut self) -> &mut Self {
        self.world.mandos_set_state(
            SetStateStep::new()
                .put_account(&self.owner_address, Account::new().nonce(1))
                .new_address(&self.owner_address, 1, &self.xswap_contract),
        );

        let ic = &self.world.interpreter_context();
        let (_new_address, ()) = self
            .xswap_contract
            .init()
            .into_blockchain_call()
            .from(self.owner_address.clone())
            .contract_code("file:output/x-swap.wasm", &ic)
            .gas_limit("50,000,000")
            .expect(TxExpect::ok().no_result())
            .execute(&mut self.world);

        self
    }
}

#[test]
fn basic_setup_test<M: ManagedTypeApi>()
{
    let _ = DebugApi::dummy();

    let mut test = XSwapTestState::setup();
    test.deploy();

    let raffle_sc_address: ManagedAddress<M> = test
        .xswap_contract
        .raffle_sc_address()
        .into_vm_query()
        .expect(TxExpect::ok().result(""))
        .execute(&mut test.world);
}

#[test]
fn test() {
    let _ = DebugApi::dummy();
    let mut world = world();
    let ctx = world.interpreter_context();

    let owner_address = "address:owner";
    let raffle_sc_address = "address:raffle_sc";
    let treasury_address = "address:treasury";
    let first_user_address = "address:user1";
    let second_user_address = "address:user2";

    let deadline: u64 = 7 * 24 * 60 * 60; // 1 week in seconds
    let wegld_id_value = "WEGLD-123456"; // when passing as argument
    let wegld_id = "str:WEGLD-123456"; // when specifying the token transfer
    let usdc_id = "str:USDC-123456";

    let mut project_sc = ContractInfo::<x_swap::Proxy<DebugApi>>::new("sc:xswap");

    // setup owner and crowdfunding SC
    world.mandos_set_state(
        SetStateStep::new()
            .put_account(owner_address, Account::new())
            .new_address(owner_address, 0, &project_sc),
    );
    let (_, ()) = project_sc
        .init()
        .into_blockchain_call()
        .from(owner_address)
        .contract_code("file:output/x-swap.wasm", &ctx)
        .gas_limit("50,000,000")
        .expect(TxExpect::ok().no_result())
        .execute(&mut world);
}