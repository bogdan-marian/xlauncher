use elrond_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("");

    blockchain.register_contract_builder(
        "file:output/xlauncher-presale.wasm",
        xlauncher_presale::ContractBuilder,
    );

    blockchain.register_contract_builder(
        "file:../xlauncher-staking-contract/output/xlauncher-staking.wasm",
        xlauncher_staking::ContractBuilder,
    );

    blockchain
}

#[test]
fn buy_ok_scen_01() {
    elrond_wasm_debug::mandos_rs("../mandos/03-buy-ok.scen", world());
}

#[test]
fn test_pricing_for_presale_round_2() {
    elrond_wasm_debug::mandos_rs("../mandos/08-test-pricing-for-presale-round-2.scen", world());
}

#[test]
fn test_stake_and_buy_ok() {
    elrond_wasm_debug::mandos_rs("mandos/09-stake-and-buy-ok.scen.json", world());
}

#[test]
fn test_stake_and_buy_ok_009() {
    elrond_wasm_debug::mandos_rs("mandos/009-stake-and-buy-ok.scen.json", world());
}

#[test]
fn test_stake_and_buy_ok_10() {
    elrond_wasm_debug::mandos_rs("mandos/010-stake-and-buy-ok.scen.json", world());
}

#[test]
fn test_stake_and_buyback_ok_012() {
    elrond_wasm_debug::mandos_rs("mandos/012-stake-and-buyback-max-allowed-ok.scen.json", world());
}

