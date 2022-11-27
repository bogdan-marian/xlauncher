PROXY=https://testnet-gateway.elrond.com
CHAIN_ID="T"
WALLET="./wallets/eve.pem"
ADDRESS=$(erdpy data load --key=address-testnet)
######################################################################

RAFFLE_SC_ADDRESS="erd1qqqqqqqqqqqqqpgqz9gpnvxsh3kep8s4zulzxwmrv6jtk39rqqesahxreh"
RAFFLE_SC_ADDRESS_HEX="0x$(erdpy wallet bech32 --decode ${RAFFLE_SC_ADDRESS})"
TREASURY_ADDRESS="erd1ygdttzrulwfspme2s4qrx5y2qyfqalju0k2vcyy8z3979whlj9qssl5uay"
TREASURY_ADDRESS_HEX="0x$(erdpy wallet bech32 --decode ${TREASURY_ADDRESS})"
RAFFLE_SC_FEE=500 # 5%
TREASURY_FEE=500 # 5%
WEGLD_TOKEN_ID="WEGLD-123456"
WEGLD_TOKEN_ID_HEX="0x$(echo -n ${WEGLD_TOKEN_ID} | xxd -p -u | tr -d '\n')"
EGLD_BASE_AMOUNT_FOR_INCENTIVE=500000000000000000 # 0.5 EGLD
INCENTIVE_TOKEN_ID="XRF-c0c96a"
INCENTIVE_TOKEN_ID_HEX="0x$(echo -n ${INCENTIVE_TOKEN_ID} | xxd -p -u | tr -d '\n')"
INCENTIVE_BASE_AMOUNT=1000000000000000000 # 1 XRF
USDC_TOKEN_ID="USDC-123456"
USDC_TOKEN_ID_HEX="0x$(echo -n ${USDC_TOKEN_ID} | xxd -p -u | tr -d '\n')"
###

deploy() {
    erdpy --verbose contract deploy  --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --outfile="deploy-testnet.interaction.json" \
    --metadata-payable \
    --gas-limit=100000000
    
    ADDRESS=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-testnet --value=${ADDRESS}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --outfile="upgrade.json" --proxy=${PROXY} --chain=${CHAIN_ID} \
    --metadata-payable \
    --gas-limit=100000000
}

setSettings() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=8000000 \
    --function="setSettings" \
    --arguments ${RAFFLE_SC_ADDRESS_HEX} ${TREASURY_ADDRESS_HEX} ${RAFFLE_SC_FEE} ${TREASURY_FEE} ${WEGLD_TOKEN_ID_HEX} ${EGLD_BASE_AMOUNT_FOR_INCENTIVE} ${INCENTIVE_TOKEN_ID_HEX} ${INCENTIVE_BASE_AMOUNT} "0x" "0x"
}

addATokens() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=8000000 \
    --function="addATokens" \
    --arguments ${WEGLD_TOKEN_ID_HEX}
}

addBTokens() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=8000000 \
    --function="addBTokens" \
    --arguments ${USDC_TOKEN_ID_HEX}
}