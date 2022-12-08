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
	DefaultSmartContractController,
	CodeMetadata,
} from "@elrondnetwork/erdjs/out";
import {
	EXPLORER_URL,
	RAFFLE_TICKET_PRICE,
	XRF_TOKEN_ID,
	RAFFLE_NUMBER_OF_WINNERS,
	RAFFLE_PRIZE_PERCENTAGES,
	XRAFFLE_SC_ADDRESS,
	COMMON_GAS_LIMIT,
	TREASURY_ADDRESS,
	XRAFFLE_SC_WASM_URL,
	XRAFFLE_SC_ABI_URL,
	XRAFFLE_SC_NAME,
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
} from './util';
import { loadContractCode } from '@elrondnetwork/erdjs/out/testutils';

async function deploy() {
	const abiRegistry = await AbiRegistry.load({ files: [XRAFFLE_SC_ABI_URL] });
	const abi = new SmartContractAbi(abiRegistry, [XRAFFLE_SC_NAME]);
	const contract = new SmartContract({ address: new Address(XRAFFLE_SC_ADDRESS), abi: abi });
	const controller = new DefaultSmartContractController(abi, provider);
	
	const tx = contract.upgrade({
		code: await loadContractCode(XRAFFLE_SC_WASM_URL),
		gasLimit: new GasLimit(100_000_000),
		initArguments: [],
		codeMetadata: new CodeMetadata(
			true,	// Upgradable
			true, // Readable
			false, // Payable
			false, // PayableBySC
		),
	});

	tx.setNonce(account.getNonceThenIncrement());
	await signer.sign(tx);
	const { transactionOnNetwork } = await controller.deploy(tx);
	console.log(`${EXPLORER_URL}${transactionOnNetwork.hash.toString()}`);

	const deployedAddress = new Address(transactionOnNetwork.logs.events[0].topics[0].valueOf());
	console.log('Upgraded Address: ', deployedAddress.bech32());
}


(async function() {
	await account.sync(provider);
	await deploy();
})();