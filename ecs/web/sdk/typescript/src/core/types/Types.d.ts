import { Keypair, PublicKey } from "@solana/web3.js";

export enum ERpcUrl {
	Devnet = "https://api.devnet.solana.com",
	Mainnet = "https://api.mainnet-beta.solana.com",
	Testnet = "https://api.testnet.solana.com",
	Local = "http://127.0.0.1:8899",
}

export interface IBTreeMap {
	K: string;
	V: string;
	A: string;
}

export interface IBluePrint {
	name: string;
	description: string;
	entities: IBTreeMap;
	regions: IBTreeMap;
	instances: IBTreeMap;
}

export interface IStorage {
	blueprint: string;
	programId: PublicKey | string;
	signer: Keypair;
	rpcUrl: string;
}

export interface ISigner {
	publicKey: PublicKey;
	secretKey: Uint8Array;
}
export interface ITsSdkParams {
	rpcUrl: string;
	programId: PublicKey | string;
	blueprintPath: string;
	keypair: Keypair;
}

declare global {
	interface Window {
		solana?: {
			isPhantom: boolean;
			connect: () => Promise<{ publicKey: PublicKey }>;
			signAndSendTransaction: (
				transaction: Transaction,
			) => Promise<{ signature: string }>;
		};
	}
}

// Possible to make this a class?
export interface Entity {
	id: string;
	data: Map<string, string>;
	name: string;
	description: string;
	properties: Map<string, string>;
}

export interface Region {
	name: string;
	description: string;
	properties: Map<string, string>;
}
