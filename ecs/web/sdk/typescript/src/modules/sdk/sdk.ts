import { TSSDKParams } from "../../types";
import * as solanaWeb3 from "@solana/web3.js";
import { Solana } from "../storage";

class TSSDK {
	// Imporant variables
	private keypair: solanaWeb3.Keypair;
	private storage: Solana;

	constructor({ rpc_url, program_id, blueprint_path, keypair_string }: TSSDKParams) {
		// Initialzation of the instance
		const secretKey = Uint8Array.from(JSON.parse(keypair_string)); // This will be a keypair_string since
		this.keypair = solanaWeb3.Keypair.fromSecretKey(secretKey);

		const programIdKey = new solanaWeb3.PublicKey(program_id);

		this.storage = new Solana({ blueprint: blueprint_path, program_id: programIdKey, rpc_url, signer: this.keypair });
	}
}
