import { Keypair, PublicKey } from "@solana/web3.js";

export enum rpcUrl {
	Devnet = "https://api.devnet.solana.com",
	Mainnet = "https://api.mainnet-beta.solana.com",
	Testnet = "https://api.testnet.solana.com",
	Local = "http://127.0.0.1:8899",
}

export interface bTreeMap {
	K: string;
	V: string;
	A: string;
}

export interface bluePrint {
	name: string;
	description: string;
	entities: bTreeMap;
	regions: bTreeMap;
	instances: bTreeMap;
}

export interface solanaStorage {
	blueprint: string;
	program_id: PublicKey | string;
	signer: Keypair;
	rpc_url: string;
}

export interface Signer {
	publicKey: PublicKey;
	secretKey: Uint8Array;
}
export interface TSSDKParams {
	rpc_url: string;
	program_id: PublicKey | string;
	blueprint_path: string;
	keypair_base58: string;
}
