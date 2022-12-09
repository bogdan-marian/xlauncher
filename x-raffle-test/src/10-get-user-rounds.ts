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
	START_ROUND_ID,
	END_ROUND_ID,
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
	const args: TypedValue[] = [
		new AddressValue(account.address),
		new U32Value(START_ROUND_ID),
		new U32Value(END_ROUND_ID),
	];
	const interaction = contractInteractor.contract.methods.getUserRounds(args);
	const res = await contractInteractor.controller.query(interaction);

	if (!res || !res.returnCode.isSuccess()) return;
	const values = res.firstValue.valueOf();
	const userRounds = values.map(value => ({
    ticket_numbers: value.ticket_numbers.map(v => v.toNumber()),
		win_ticket_number: value.win_ticket_number.toNumber(),
    prize_ranking: value.prize_ranking.toNumber(),
    prize_claimed: value.prize_claimed,
	}));

	console.log('userRounds: ', userRounds);
}


(async function() {
	await account.sync(provider);
	await main();
})();