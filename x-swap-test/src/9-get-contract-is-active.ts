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
} from "./config";

import {
	account,
	provider,
	signer,
	getXSwapContractInteractor,
} from './provider';
import BigNumber from 'bignumber.js';
import {
	sleep,
	convertBigNumberToDate,
	convertWeiToEsdt,
	convertEsdtToWei,
} from './util';
import { ValidatorPublicKey } from '@elrondnetwork/erdjs-walletcore/out';

async function main() {
	const contractInteractor = await getXSwapContractInteractor();
	let interaction = contractInteractor.contract.methods.getContractIsActive();
	let res = await contractInteractor.controller.query(interaction);

	if (!res || !res.returnCode.isSuccess()) return;
	const value = res.firstValue.valueOf();
	console.log('ContractIsActive: ', value);
}

(async function() {
	await account.sync(provider);
	await main();
})();