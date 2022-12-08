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
	let interaction = contractInteractor.contract.methods.getCurrentRoundStatus();
	let res = await contractInteractor.controller.query(interaction);

	if (!res || !res.returnCode.isSuccess()) return;
	let value = res.firstValue.valueOf();
	const currentRoundId = value.toNumber();
	console.log('Current Round ID: ', currentRoundId);

	const args = [new U32Value(currentRoundId)];
	interaction = contractInteractor.contract.methods.getRound(args);
	res = await contractInteractor.controller.query(interaction);

	if (!res || !res.returnCode.isSuccess()) return;
	value = res.firstValue.valueOf();
	const currentRound = {
		round_id: value.round_id.toNumber(),
    round_status: value.round_status,
    round_start_timestamp: new Date(value.round_start_timestamp.toNumber() * 1000),
    round_end_timestamp: new Date(value.round_end_timestamp.toNumber() * 1000),
    round_ticket_token: value.round_ticket_token.toString(),
    round_ticket_price: convertWeiToEsdt(value.round_ticket_price),

    round_prize_tokens: value.round_prize_tokens,
    round_left_tokens: value.round_left_tokens,

    round_number_of_winners: value.round_number_of_winners.toNumber(),
    round_prize_percentages: value.round_prize_percentages.map(v => v.toNumber()),
    round_winners: value.round_winners.map(v => v.bech32()),
    round_win_numbers: value.round_win_numbers.map(v => v.toNumber()),

    round_sold_tickets: value.round_sold_tickets.toNumber(),
    round_sold_amount: convertWeiToEsdt(value.round_sold_amount),
	};

	console.log('Current Round: ', currentRound);
}


(async function() {
	await account.sync(provider);
	await main();
})();