use multiversx_sc_scenario::*;

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
