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
	BytesValue,
	BigUIntValue,
	Egld,
	U32Value,
	NetworkConfig,
	TypedValue,
	ArgSerializer,
	TransactionPayload,
	Transaction,
	Interaction,
	DefaultSmartContractController,
} from "@elrondnetwork/erdjs/out";
import { UserSecretKey, UserSigner } from "@elrondnetwork/erdjs-walletcore"
import {
	GATEWAY_URL,
	PEM_PATH,
	EXPLORER_URL,
	CHAIN_ID,
	XRAFFLE_SC_ABI_URL,
	XRAFFLE_SC_NAME,
	XRAFFLE_SC_ADDRESS,
} from "./config";

export const provider = new ProxyProvider(GATEWAY_URL, { timeout: 20000 });
let config = NetworkConfig.getDefault();
config.ChainID = new ChainID(CHAIN_ID);
config.sync(provider);

const pem = fs.readFileSync(PEM_PATH, { encoding: 'utf-8' }).trim();
export const signer = UserSigner.fromPem(pem);
export const account = new Account(new Address((signer.getAddress()).bech32()));

export const getXRaffleContractInteractor = async () => {
	const registry = await AbiRegistry.load({ files: [XRAFFLE_SC_ABI_URL] });
	const abi = new SmartContractAbi(registry, [XRAFFLE_SC_NAME]);
	const contract = new SmartContract({ address: new Address(XRAFFLE_SC_ADDRESS), abi: abi });
	const controller = new DefaultSmartContractController(abi, provider);

	return {
		contract,
		controller,
	};
};