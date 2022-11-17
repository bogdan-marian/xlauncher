use x_swap::*;
use elrond_wasm::types::{BigUint, Address, TokenIdentifier, ManagedVec};
use elrond_wasm_debug::{DebugApi, testing_framework::{BlockchainStateWrapper, ContractObjWrapper}, rust_biguint};

const WASM_PATH: &'static str = "output/x-swap.wasm";

const WEGLD_ID: &[u8] = b"WEGLD-123456";
const USDC_ID: &[u8] = b"USDC-123456";
const DEADLINE: u64 = 7 * 24 * 60 * 60; // 1 week in seconds
const RAFFLE_SC_FEE: u64 = 500; // 5%
const TREASURY_FEE: u64 = 500; // 5%
const TOTAL_PERCENTAGE: u64 = 100_00; // 100%
const EGLD_BASE_AMOUNT_FOR_INCENTIVE: u64 = 5;
const INCENTIVE_TOKEN_ID: &[u8] = b"XRF-123456";
const INCENTIVE_BASE_AMOUNT: u64 = 1;

struct XSwapSetup<XSwapObjBuilder>
where XSwapObjBuilder: 'static + Copy + Fn() -> x_swap::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub first_user_address: Address,
    pub second_user_address: Address,
    pub raffle_sc_address: Address,
    pub treasury_address: Address,
    pub project_wrapper: ContractObjWrapper<x_swap::ContractObj<DebugApi>, XSwapObjBuilder>,
}

fn setup_project<XSwapObjBuilder>(
    project_builder: XSwapObjBuilder,
) -> XSwapSetup<XSwapObjBuilder>
where
    XSwapObjBuilder: 'static + Copy + Fn() -> x_swap::ContractObj<DebugApi>,
{
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_zero);
    let raffle_sc_address = blockchain_wrapper.create_user_account(&rust_zero);
    let treasury_address = blockchain_wrapper.create_user_account(&rust_zero);

    let first_user_address = blockchain_wrapper.create_user_account(&rust_zero);
    let second_user_address = blockchain_wrapper.create_user_account(&rust_zero);
    let project_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        project_builder,
        WASM_PATH,
    );

    blockchain_wrapper.set_esdt_balance(&first_user_address, WEGLD_ID, &rust_biguint!(1_000));
    blockchain_wrapper.set_esdt_balance(&second_user_address, USDC_ID, &rust_biguint!(1_000));

    blockchain_wrapper
        .execute_tx(&owner_address, &project_wrapper, &rust_zero, |sc| {
            sc.init();
        })
        .assert_ok();

    blockchain_wrapper.add_mandos_set_account(project_wrapper.address_ref());

    XSwapSetup {
        blockchain_wrapper,
        owner_address,
        first_user_address,
        second_user_address,
        raffle_sc_address,
        treasury_address,
        project_wrapper,
    }
}

#[test]
fn init_test() {
    let project_setup = setup_project(x_swap::contract_obj);
    project_setup
        .blockchain_wrapper
        .write_mandos_output("_generated_init.scen.json");
}

#[test]
fn set_settings_test() {
    let mut project_setup = setup_project(x_swap::contract_obj);
    let b_wrapper = &mut project_setup.blockchain_wrapper;
    let owner_address = &project_setup.owner_address;
    let raffle_sc_address = &project_setup.raffle_sc_address;
    let treasury_address = &project_setup.treasury_address;

    b_wrapper
        .execute_tx(
            owner_address,
            &project_setup.project_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.set_settings(
                    raffle_sc_address,
                    treasury_address,
                    RAFFLE_SC_FEE,
                    TREASURY_FEE,
                    TokenIdentifier::from_esdt_bytes(WEGLD_ID),
                    rust_biguint!(EGLD_BASE_AMOUNT_FOR_INCENTIVE),
                    TokenIdentifier::from_esdt_bytes(INCENTIVE_TOKEN_ID),
                    rust_biguint!(INCENTIVE_BASE_AMOUNT),
                    ManagedVec::from_single_item(TokenIdentifier::from_esdt_bytes(WEGLD_ID)),
                    ManagedVec::from_single_item(TokenIdentifier::from_esdt_bytes(USDC_ID)),
                );
            },
        )
        .assert_ok();

    project_setup
        .blockchain_wrapper
        .write_mandos_output("_generated_fund.scen.json");
}