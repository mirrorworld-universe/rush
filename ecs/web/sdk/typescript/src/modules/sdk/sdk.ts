import { TSSDKParams } from "../../types";
import { Solana } from "../storage";
import { Keypair, PublicKey } from "@solana/web3.js";
import bs58 from "bs58";

export class RushSDK {
	// Imporant variables
	private keypair: Keypair;
	private storage: Solana;

	constructor({ rpc_url, program_id, blueprint_path, keypair_base58 }: TSSDKParams) {
		// Initialzation of the instance
		const secretKey = bs58.decode(keypair_base58);
		this.keypair = Keypair.fromSecretKey(secretKey);

		const programIdKey = new PublicKey(program_id);

		this.storage = new Solana({ blueprint: blueprint_path, program_id: programIdKey, rpc_url, signer: this.keypair });
	}
}
