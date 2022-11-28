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

async function setSettings() {
	const args: TypedValue[] = [
		new AddressValue(new Address(XRAFFLE_SC_ADDRESS)), 	// receiver
		new U32Value(1), 																		//number of tokens to transfer
		
		BytesValue.fromUTF8(USDC_TOKEN_ID),									// token id
		new U32Value(0),																		// token nonce
		new BigUIntValue(convertEsdtToWei(3_000, USDC_TOKEN_DECIMALS)),
		
		BytesValue.fromUTF8('injectPrize'),
	];
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const dataString = `MultiESDTNFTTransfer@${argumentsString}`;
	const data = new TransactionPayload(dataString);

	const tx = new Transaction({
			nonce: account.getNonceThenIncrement(),
			receiver: account.address,
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