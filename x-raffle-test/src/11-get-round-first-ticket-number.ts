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

async function getRoundFirstTicketNumber(roundId: number) {
	const contractInteractor = await getXRaffleContractInteractor();
	const args = [new U32Value(roundId)];
	const interaction = contractInteractor.contract.methods.getRoundFirstTicketNumber(args);
	const res = await contractInteractor.controller.query(interaction);

	if (!res || !res.returnCode.isSuccess()) return;
	const value = res.firstValue.valueOf();
	const roundFirstTicketNumber = value.toNumber();
	
	console.log(`Round ${roundId}: ${roundFirstTicketNumber}`);
}


(async function() {
	await account.sync(provider);
	
	for (let roundId = 1; roundId < 10; roundId++) {
		await getRoundFirstTicketNumber(roundId);
	}
})();