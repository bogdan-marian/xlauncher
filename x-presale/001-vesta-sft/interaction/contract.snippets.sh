PROJECT="${PWD}"

#possible pem values
#devnet_owner_wallet.pem
#testnet_owner_wallet.pem
#mainnet_owner_wallet.pem

#address key values: this is the same for all networks

#deploy transaction values: this is the same for all networks
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)

MY_DECIMALS="000000000000000000"

#devnet proxy and chain
#devnet=https://devnet-gateway.elrond.com
#testnet=https://testnet-gateway.elrond.com
#mainnet=https://mainnet-gateway.elrond.com

#chain values: D, T, M
CURRENT_ENV="not-set"
MY_LOGS="interaction-logs"
#envs logs values: devnet, testnet, mainnet
#token id values: devnet=XLH-4f55ab, testnet=XLH-0be7d1, mainnet=XLH-8daa50

INITIAL_PRICE="1000${MY_DECIMALS}"
MIN_AMOUNT="1250000000000000000"
MAX_AMOUNT="25000000000000000000000"
MAX_BALANCE="25000000000000000000000"
CLIENT_PEM="${PROJECT}/../../wallets/users/client1.pem"
setEnvDevnet() {
  cp -f erdpy.data-storage-devnet.json erdpy.data-storage.json
  CURRENT_ENV="devnet"
  PEM_FILE="${PROJECT}/../../wallets/users/devnet_owner_wallet.pem"
  ADDRESS=$(erdpy data load --key=address-devnet)
  PROXY=https://devnet-gateway.elrond.com
  CHAINID=D
  ENV_LOGS="devnet"
  TOKEN_ID="XLH-4a7cc0"
  TOKEN_ID_HEX=$(echo -n ${TOKEN_ID} | xxd -p)
  # erdpy wallet bech32 --decode erd1qqqqqqqqqqqqqpgqxmeg3k0ty84hm3f8n9wdfpukspc0asj3pa7qtt6j0t
  #  STAKING_ADDRESS="0x0000000000000000050036f288d9eb21eb7dc527995cd487968070fec2510f7c"
  STAKING_ADDRESS="0x000000000000000005002edbe96b459ee68341029841b9d1f8076b1d46e30f7c"
}

setEnvTestnet() {
  cp -f erdpy.data-storage-testnet.json erdpy.data-storage.json
  CURRENT_ENV="testnet"
  PEM_FILE="${PROJECT}/../../wallets/users/testnet_owner_wallet.pem"
  ADDRESS=$(erdpy data load --key=address-devnet)
  PROXY=https://testnet-gateway.elrond.com
  CHAINID=T
  ENV_LOGS="testnet"
  TOKEN_ID="Z2I-ffb426"
  TOKEN_ID_HEX=$(echo -n ${TOKEN_ID} | xxd -p)
  # erdpy wallet bech32 --decode erd1qqqqqqqqqqqqqpgqdw9gatcwzlsdtvjwu3mveln57g0quyzzpa7q9jg84s
  STAKING_ADDRESS="0x000000000000000005006b8a8eaf0e17e0d5b24ee476ccfe74f21e0e10420f7c"
}

setEnvMainnet() {
  cp -f erdpy.data-storage-mainnet.json erdpy.data-storage.json
  CURRENT_ENV="mainnet"
  PEM_FILE="${PROJECT}/../../wallets/users/mainnet_owner_wallet.pem"
  ADDRESS=$(erdpy data load --key=address-devnet)
  PROXY=https://api.elrond.com
  CHAINID=1
  ENV_LOGS="mainnet"
  #TOKEN_ID="BCOIN-efba9c"
  TOKEN_ID="Z2I-e9f0ff"
  TOKEN_ID_HEX=$(echo -n ${TOKEN_ID} | xxd -p)
  # erdpy wallet bech32 --decode erd1qqqqqqqqqqqqqpgql0yx4uetca8g4wmr96d9z7094vj3uhpt4d6qm5srfk
  STAKING_ADDRESS="0x00000000000000000500fbc86af32bc74e8abb632e9a5179e5ab251e5c2bab74"
}

printCurrentEnv() {
  echo ${CURRENT_ENV}
}

deploy() {
  erdpy --verbose contract deploy --project=${PROJECT} --recall-nonce --pem=${PEM_FILE} \
    --gas-limit=50000000 --send --outfile="${MY_LOGS}/deploy-${ENV_LOGS}.json" \
    --proxy=${PROXY} --chain=${CHAINID} \
    --arguments "0x${TOKEN_ID_HEX}" ${INITIAL_PRICE} ${MIN_AMOUNT} ${MAX_AMOUNT} ${MAX_BALANCE} ${STAKING_ADDRESS} || return

  TRANSACTION=$(erdpy data parse --file="${MY_LOGS}/deploy-${ENV_LOGS}.json" --expression="data['emitted_tx']['hash']")
  ADDRESS=$(erdpy data parse --file="${MY_LOGS}/deploy-${ENV_LOGS}.json" --expression="data['emitted_tx']['address']")

  erdpy data store --key=address-devnet --value=${ADDRESS}
  erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

  echo ""
  echo "Smart contract address: ${ADDRESS}"
}

updateContract() {
  erdpy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${PEM_FILE} \
    --gas-limit=50000000 --send --outfile="${MY_LOGS}/deploy-${ENV_LOGS}.json" \
    --proxy=${PROXY} --chain=${CHAINID} \
    --arguments "0x${TOKEN_ID_HEX}" ${INITIAL_PRICE} ${MIN_AMOUNT} ${MAX_AMOUNT} ${MAX_BALANCE} ${STAKING_ADDRESS}
}

fundContract() {
  method_name="0x$(echo -n 'fundContract' | xxd -p -u | tr -d '\n')"
  token_id="0x$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"
  #  amount="2000001${MY_DECIMALS}"
  amount="2000001${MY_DECIMALS}"
  erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=3000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="ESDTTransfer" \
    --arguments $token_id $amount $method_name \
    --send \
    --outfile="${MY_LOGS}/fundContract-${ENV_LOGS}.json"
}

getTokenBalance() {
  erdpy --verbose contract query ${ADDRESS} --function="getTokenBalance" \
    --proxy=${PROXY}
}

getPrice() {
  erdpy --verbose contract query ${ADDRESS} --function="getPrice" \
    --proxy=${PROXY}
}

getMinAmount() {
  erdpy --verbose contract query ${ADDRESS} --function="getMinAmount" \
    --proxy=${PROXY}
}

getMaxAmount() {
  erdpy --verbose contract query ${ADDRESS} --function="getMaxAmount" \
    --proxy=${PROXY}
}

getMaxBalance() {
  erdpy --verbose contract query ${ADDRESS} --function="getMaxBalance" \
    --proxy=${PROXY}
}

buyTokens() {
  erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${CLIENT_PEM} \
    --gas-limit=100000000 \
    --function="buy" \
    --value=1250000000000000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --send \
    --outfile="${MY_LOGS}/buyTokens-${ENV_LOGS}.json"
}

collect() {
  erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${PEM_FILE} \
    --gas-limit=5000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="collect" \
    --send \
    --outfile="${MY_LOGS}/fundContract-${ENV_LOGS}.json"
}

switchSellIsActiveToFalse(){
  erdpy --verbose contract call ${ADDRESS} --recall-nonce \
      --pem=${PEM_FILE} \
      --gas-limit=8000000 \
      --proxy=${PROXY} --chain=${CHAINID} \
      --function="switchSellIsActiveToFalse" \
      --send \
      --outfile="${MY_LOGS}/switchSellIsActiveToFalse-${ENV_LOGS}.json"
}

switchSellIsActiveToTrue(){
  erdpy --verbose contract call ${ADDRESS} --recall-nonce \
      --pem=${PEM_FILE} \
      --gas-limit=8000000 \
      --proxy=${PROXY} --chain=${CHAINID} \
      --function="switchSellIsActiveToTrue" \
      --send \
      --outfile="${MY_LOGS}/switchSellIsActiveToTrue-${ENV_LOGS}.json"
}

buyback() {
  method_name="0x$(echo -n 'buyback' | xxd -p -u | tr -d '\n')"
  token_id="0x$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"
  #  amount="2000001${MY_DECIMALS}"
  amount="1${MY_DECIMALS}"
  erdpy --verbose contract call ${ADDRESS} --recall-nonce \
    --pem=${CLIENT_PEM} \
    --gas-limit=5000000 \
    --proxy=${PROXY} --chain=${CHAINID} \
    --function="ESDTTransfer" \
    --arguments $token_id $amount $method_name \
    --send \
    --outfile="${MY_LOGS}/buyback-${ENV_LOGS}.json"
}

getClientBoughtValue() {
  # clinet address
  # erdpy wallet bech32 --decode erd1m98v82l4vkkejlwjka944r7nhlrf8j4xjefw03m0d2tzt5pywgyqfsq39v
  timestamp=$(date +%s)
  echo "query timestamp=${timestamp}"
  erdpy --verbose contract query ${ADDRESS} --function="getClientBoughtValue" \
    --arguments 0xd94ec3abf565ad997dd2b74b5a8fd3bfc693caa69652e7c76f6a9625d0247208 \
    --proxy=${PROXY}
}

getClientBuybackValue() {
  # erdpy wallet bech32 --decode erd1m98v82l4vkkejlwjka944r7nhlrf8j4xjefw03m0d2tzt5pywgyqfsq39v
  timestamp=$(date +%s)
  echo "query timestamp=${timestamp}"
  erdpy --verbose contract query ${ADDRESS} --function="clientBuybackValue" \
    --arguments 0xd94ec3abf565ad997dd2b74b5a8fd3bfc693caa69652e7c76f6a9625d0247208 \
    --proxy=${PROXY}
}
