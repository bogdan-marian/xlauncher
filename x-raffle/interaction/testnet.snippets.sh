cp -f erdpy.data-storage-testnet.json erdpy.data-storage.json
CURRENT_ENV="testnet"
PROXY=https://testnet-gateway.elrond.com
CHAIN_ID="T"
WALLET="../../wallets/testnet_owner_wallet.pem"
ADDRESS=$(erdpy data load --key=address-devnet)
######################################################################
LOGS_FOLDER="interaction-logs"
TICKET_TOKEN="XRF-0a31bd"
TICKET_TOKEN_HEX="0x$(echo -n ${TICKET_TOKEN} | xxd -p -u | tr -d '\n')"

# 01 december 2022
END_TIMESTAMP=$(date -d '2022-12-01 21:00:00' +"%s")
TICKET_PRICE=100000000000000000 # 0.1 ESDT
NUMBER_OF_WINNERS=5
PRIZE_PERCENTAGES="4000 2500 2000 1000 500"

###

deploy() {
  erdpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --outfile="${LOGS_FOLDER}/deploy-testnet.interaction.json" \
    --metadata-payable \
    --gas-limit=80000000

  ADDRESS=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['contractAddress']")

  erdpy data store --key=address-devnet --value=${ADDRESS}

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

sendTokensToClients() {
  ALICE_ADDRESS="erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th"
  BOB_ADDRESS="erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx"
  CAROL_ADDRESS="erd1k2s324ww2g0yj38qn2ch2jwctdy8mnfxep94q9arncc6xecg3xaq6mjse8"
  DAN_ADDRESS="erd1kyaqzaprcdnv4luvanah0gfxzzsnpaygsy6pytrexll2urtd05ts9vegu7"
  EVE_ADDRESS="erd18tudnj2z8vjh0339yu3vrkgzz2jpz8mjq0uhgnmklnap6z33qqeszq2yn4"
  FRANK_ADDRESS="erd1kdl46yctawygtwg2k462307dmz2v55c605737dp3zkxh04sct7asqylhyv"
  CLIENT_ADDRESS=${FRANK_ADDRESS}
  DENOMINATION="000000000000000000"
  erdpy --verbose contract call ${CLIENT_ADDRESS} --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=1000000 \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --function="ESDTTransfer" \
    --arguments ${TICKET_TOKEN_HEX} "1${DENOMINATION}" \
    --send \
    --outfile="${LOGS_FOLDER}/sendTokensToClients.json"
}

buyTickets() {
  ALICE_WALLET="../../wallets/users/alice.pem"
  BOB_WALLET="../../wallets/users/bob.pem"
  CAROL_WALLET="../../wallets/users/carol.pem"
  DAN_WALLET="../../wallets/users/dan.pem"
  EVE_WALLET="../../wallets/users/eve.pem"
  FRANK_WALLET="../../wallets/users/frank.pem"
  CLIENT_WALLET=${FRANK_WALLET}
  DENOMINATION="000000000000000000"
  method_name="0x$(echo -n 'buyTickets' | xxd -p -u | tr -d '\n')"
  number_of_tikets="1"
  erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${CLIENT_WALLET} \
    --gas-limit=10000000 \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --function="ESDTTransfer" \
    --arguments ${TICKET_TOKEN_HEX} "${number_of_tikets}${DENOMINATION}" ${method_name} ${number_of_tikets} \
    --send \
    --outfile="${LOGS_FOLDER}/buyTickets.json"
}

getCurrentRound() {
  erdpy --verbose contract query ${ADDRESS} --function="getCurrentRound" \
    --proxy=${PROXY}
}

getCurrentRoundStatus() {
  erdpy --verbose contract query ${ADDRESS} --function="getCurrentRoundStatus" \
    --proxy=${PROXY}
}

getRound() {
  erdpy --verbose contract query ${ADDRESS} --function="getRound" \
    --arguments 2 \
    --proxy=${PROXY}
}

getRoundUserTickets(){
  erdpy --verbose contract query ${ADDRESS} --function="getRoundUserTickets" \
      --arguments 2 \
      --proxy=${PROXY}
}

getRoundUserTickets() {
  erdpy --verbose contract query ${ADDRESS} --function="getRoundUserTickets" \
    --arguments 2 \
    --proxy=${PROXY}
}

finishRound() {
  erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=5000000 \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --function="finishRound" \
    --send \
    --outfile="${LOGS_FOLDER}/finishRound.json"
}