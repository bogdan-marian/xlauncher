// export const GATEWAY_URL = "https://devnet-gateway.elrond.com";
// export const API_URL = "https://devnet-api.elrond.com";
// export const EXPLORER_URL = 'https://devnet-explorer.elrond.com/transactions/';
// export const CHAIN_ID = "D"

export const GATEWAY_URL = "https://testnet-gateway.elrond.com";
export const API_URL = "https://testnet-api.elrond.com";
export const EXPLORER_URL = 'https://testnet-explorer.elrond.com/transactions/';
export const CHAIN_ID = "T"

// export const GATEWAY_URL = "https://gateway.elrond.com";
// export const API_URL = "https://api.elrond.com";
// export const EXPLORER_URL = 'https://explorer.elrond.com/transactions/';
// export const CHAIN_ID = "1"

 export const PEM_PATH = "./wallets/eve.pem";
// export const PEM_PATH = "./wallets/frank.pem";
// export const PEM_PATH = "./wallets/grace.pem";
// export const PEM_PATH = "./wallets/heidi.pem";

export const MAX_GAS_PER_TRANSACTIONS = 600_000_000;
export const DELAY_TIME = 100;
export const COMMON_GAS_LIMIT = 60_000_000;

// 1 - Raffle
// SC metadata
export const XRAFFLE_SC_ABI_URL = 'abi/x-raffle.abi.json';
export const XRAFFLE_SC_WASM_URL = 'abi/x-raffle.wasm';
export const XRAFFLE_SC_NAME = 'XRaffle';
export const XRAFFLE_SC_ADDRESS = 'erd1qqqqqqqqqqqqqpgqs7sraj93nrt8g00psawru0t2yp6kj3j3qqesu53gp8';

// for set-settings
export const RAFFLE_TICKET_PRICE = '5000000000000000000';
export const RAFFLE_NUMBER_OF_WINNERS = 2;
export const RAFFLE_PRIZE_PERCENTAGES = [70_00, 30_00];
export const TREASURY_ADDRESS = 'erd15936k9pw34xyzmcaupyn7lpr7f6p20q50h4wlgemxg7h9zasdfysmhg50z';

// for buy-tickets, inject-prize
export const XRF_TOKEN_ID = 'XRF-c0c96a';
export const XRF_TOKEN_DECIMALS = 18;
export const USDC_TOKEN_ID = 'USDC-ca298e';
export const USDC_TOKEN_DECIMALS = 6;

// temp variables
export const ROUND_ID = 1;
export const TICKET_NUMBER = 1;