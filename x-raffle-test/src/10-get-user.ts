import * as fs from 'fs';
import { sendTransactions, timedOutBatchTransactionsStates } from "@elrondnetwork/dapp-core";
import {
	Account,
	Address,
	AddressValue,
	ChainID,
	ContractFunction,
	GasLimit,
	I8Value,
	ProxyProvider,
	SmartContract,
	StringValue,
	AbiRegistry,
	SmartContractAbi,
	Egld,
	Balance,
	BigUIntValue,
	BytesValue,
	ArgSerializer,
	TransactionPayload,
	Transaction,
	TypedValue,
	U64Value,
	U32Value,
} from "@elrondnetwork/erdjs/out";
import {
	EXPLORER_URL,
	RAFFLE_TICKET_PRICE,
	XRF_TOKEN_ID,
	RAFFLE_NUMBER_OF_WINNERS,
	RAFFLE_PRIZE_PERCENTAGES,
	XRAFFLE_SC_ADDRESS,
	COMMON_GAS_LIMIT,
	USDC_TOKEN_ID,
	USDC_TOKEN_DECIMALS,
	XRF_TOKEN_DECIMALS,
} from "./config";

import {
	account,
	provider,
	signer,
	getXRaffleContractInteractor,
} from './provider';
import BigNumber from 'bignumber.js';
import {
	sleep,
	convertBigNumberToDate,
	convertWeiToEsdt,
	convertEsdtToWei,
} from './util';

async function main() {
	const contractInteractor = await getXRaffleContractInteractor();
	const args = [new AddressValue(account.address)];
	const interaction = contractInteractor.contract.methods.getUser(args);
	const res = await contractInteractor.controller.query(interaction);

	if (!res || !res.returnCode.isSuccess()) return;
	const value = res.firstValue.valueOf();
	const user = {
		address: value.address.bech32(),
    round_ticket_numbers: value.round_ticket_numbers.map(arr => arr.map(v => v.toNumber())),
    round_prize_rankings: value.round_prize_rankings.map(v => v.toNumber()),
    round_prize_claimed: value.round_prize_rankings.map(v => v.toNumber()),
	};

	console.log('User: ', user);
}


(async function() {
	await account.sync(provider);
	await main();
})();