PROXY=https://testnet-gateway.elrond.com
CHAIN_ID="T"
WALLET="./wallets/eve.pem"
ADDRESS=$(erdpy data load --key=address-testnet)
######################################################################

TICKET_TOKEN="XRF-123456"
TICKET_TOKEN_HEX="0x$(echo -n ${TICKET_TOKEN} | xxd -p -u | tr -d '\n')"

END_TIMESTAMP=1668826285
TICKET_PRICE=100000000000000000 # 0.1 ESDT
NUMBER_OF_WINNERS=5
PRIZE_PERCENTAGES="4000 2500 2000 1000 500"

###

deploy() {
    erdpy --verbose contract deploy  --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --outfile="deploy-testnet.interaction.json" \
    --metadata-payable \
    --gas-limit=80000000
    
    ADDRESS=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-testnet --value=${ADDRESS}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --outfile="upgrade.json" --proxy=${PROXY} --chain=${CHAIN_ID} \
    --metadata-payable \
    --gas-limit=70000000
}

setTicketToken() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=8000000 \
    --function="setTicketToken" \
    --arguments ${TICKET_TOKEN_HEX}
}

openRound() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=8000000 \
    --function="openRound" \
    --arguments ${END_TIMESTAMP} ${TICKET_PRICE} ${NUMBER_OF_WINNERS} ${PRIZE_PERCENTAGES}
}

finishRound() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=8000000 \
    --function="finishRound"
}