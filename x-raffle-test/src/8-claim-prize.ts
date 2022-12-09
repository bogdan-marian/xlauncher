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
	TICKET_NUMBER,
	ROUND_ID,
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

async function claimPrize(roundId: number, ticketNumber: number) {
	const args: TypedValue[] = [
		new U32Value(roundId),	// round_id
		new U32Value(ticketNumber),	// ticket_number
	];
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const dataString = `claimPrize@${argumentsString}`;
	const data = new TransactionPayload(dataString);

	const tx = new Transaction({
			nonce: account.getNonceThenIncrement(),
			receiver: new Address(XRAFFLE_SC_ADDRESS),
			value: Balance.Zero(),
			data: data,
			gasLimit: new GasLimit(COMMON_GAS_LIMIT),
	});

	await signer.sign(tx);
	const txHash = await tx.send(provider);
	console.log(`${EXPLORER_URL}${txHash.toString()}`);
}


(async function() {
	await account.sync(provider);

	await claimPrize(ROUND_ID, TICKET_NUMBER);
})();