import * as fs from 'fs';
import {
	Address,
	GasLimit,
	Balance,
	BigUIntValue,
	BytesValue,
	ArgSerializer,
	TransactionPayload,
	Transaction,
	TypedValue,
	OptionalValue,
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
	USDC_TOKEN_DECIMALS,
	WEGLD_TOKEN_DECIMALS,
	XRF_TOKEN_DECIMALS,
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
	convertEsdtToWei,
} from './util';

async function collectIncentiveToken() {
	// const args: TypedValue[] = [
	// 	new BigUIntValue(convertEsdtToWei(300, XRF_TOKEN_DECIMALS))	// collect_amount, OptionalValue - you can omit this value and all XRF token will be withdrew
	// ];

	// const { argumentsString } = new ArgSerializer().valuesToString(args);
	// const dataString = `collectIncentiveToken@${argumentsString}`;
	// const data = new TransactionPayload(dataString);

	const data = new TransactionPayload('collectIncentiveToken');	// collect all XRF

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
	await collectIncentiveToken();
})();