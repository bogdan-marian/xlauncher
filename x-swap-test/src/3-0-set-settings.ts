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
	createListOfTokenIdentifiers,
} from "@elrondnetwork/erdjs/out";
import {
	EXPLORER_URL,
	XRF_TOKEN_ID,
	COMMON_GAS_LIMIT,
	XSWAP_SC_ADDRESS,
	XRAFFLE_SC_ADDRESS,
	TREASURY_ADDRESS,
	RAFFLE_SC_FEE,
	TREASURY_FEE,
	WEGLD_TOKEN_ID,
	EGLD_BASE_AMOUNT_FOR_INCENTIVE,
	INCENTIVE_BASE_AMOUNT,
	USDC_TOKEN_ID,
} from "./config";

import {
	account,
	provider,
	signer,
} from './provider';
import BigNumber from 'bignumber.js';
import {
	sleep,
	convertBigNumberToDate,
	convertWeiToEsdt,
} from './util';

async function setSettings() {
	const args: TypedValue[] = [
		new AddressValue(new Address(XRAFFLE_SC_ADDRESS)),
		new AddressValue(new Address(TREASURY_ADDRESS)),
		new U32Value(RAFFLE_SC_FEE),
		new U32Value(TREASURY_FEE),
		BytesValue.fromUTF8(WEGLD_TOKEN_ID),
		new BigUIntValue(EGLD_BASE_AMOUNT_FOR_INCENTIVE),
		BytesValue.fromUTF8(XRF_TOKEN_ID),
		new BigUIntValue(INCENTIVE_BASE_AMOUNT),
	];
	args.push(createListOfTokenIdentifiers([WEGLD_TOKEN_ID]));	// a_token_list
	args.push(createListOfTokenIdentifiers([USDC_TOKEN_ID]));		// b_token_list

	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const dataString = `setSettings@${argumentsString}`;
	const data = new TransactionPayload(dataString);

	const tx = new Transaction({
			nonce: account.getNonceThenIncrement(),
			receiver: new Address(XSWAP_SC_ADDRESS),
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
	await setSettings();
})();