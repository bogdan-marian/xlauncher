use multiversx_sc::storage::mappers::SingleValue;
use multiversx_sc_scenario::*;
use multiversx_sc_scenario::{scenario_model::*};
use multiversx_sc_scenario::{*, num_bigint::BigUint, scenario_model::*};

use xlauncher_presale::ProxyTrait;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("");

    blockchain.register_contract(
        "file:output/xlauncher-presale.wasm",
        xlauncher_presale::ContractBuilder,
    );
    blockchain
}

#[test]
fn buy_ok_scen_01() {
    multiversx_sc_scenario::run_rs("scenarios/03-buy-ok.scen.json", world());
}

#[test]
fn test_pricing_for_presale_round_2() {
    multiversx_sc_scenario::run_rs("scenarios/08-test-pricing-for-presale-round-2.scen.json", world());
}

#[test]
fn test_02_fund_sft_raw() {
    let mut world = world();
    let ic = world.interpreter_context();
    let owner_address = "address:owner";
    let mut xlauncher_contract = ContractInfo::<xlauncher_presale::Proxy<DebugApi>>::new("sc:xlauncher-presale");

    let token_id = "str:BCOIN-a00000";
    let initial_price = "10,000,000,000,000,000,000,000";
    let min_amount = "250,000,000,000,000,000";
    let max_amount_val = "5,000,000,000,000,000,000";
    let max_balance_val = "55,000,000,000,000,000,000,000";

    world
        .set_state_step(
            SetStateStep::new()
                .put_account("address:owner", Account::new().nonce(0))
                .new_address("address:owner", 0, "sc:xlauncher-presale"),
        )
        .sc_deploy_step(
            ScDeployStep::new()
                .from(owner_address)
                .contract_code("file:output/xlauncher-presale.wasm", &ic)
                .call(xlauncher_contract.init(token_id, initial_price, min_amount, max_amount_val, max_balance_val))
                .gas_limit("5,000,000")
                .expect(TxExpect::ok().no_result()),
        );
}
