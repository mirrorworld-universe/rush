import { Keypair, PublicKey } from "@solana/web3.js";
import { Storage } from "./modules/storage/storage";
import { ITsSdkParams } from "./types/types";

export class RushSdk {
	private keypair: Keypair;
	private storage: Storage;

	constructor({ rpcUrl, programId, blueprintPath, secretKey }: ITsSdkParams) {
		this.keypair = Keypair.fromSecretKey(secretKey);

		const programIdKey = new PublicKey(programId);

		this.storage = new Storage({
			blueprint: blueprintPath,
			programId: programIdKey,
			rpcUrl,
			signer: this.keypair,
		});
	}

	public async set(entityId: string, data: any) {
		try {
			const signature = await this.storage.set(entityId, data);
			console.log("Set function executed successfully. Signature:", signature);
			return signature;
		} catch (error) {
			console.error("Error in set function:", error);
			throw error;
		}
	}

	public async get(entityId: string) {
		try {
			const data = await this.storage.get(entityId);
			console.log("Get function executed successfully. Data:", data);
			return data;
		} catch (error) {
			console.error("Error in get function:", error);
			throw error;
		}
	}

	public async delete(entityId: string) {
		try {
			const result = await this.storage.delete(entityId);
			console.log("Delete function executed successfully. Result:", result);
			return result;
		} catch (error) {
			console.error("Error in delete function:", error);
			throw error;
		}
	}

	public create() {
		this.storage.create();
	}

	public migrate() {
		this.storage.migrate();
	}

	public signin(): Keypair {
		const keypair = Keypair.generate();
		console.log("New Keypair Created:");
		console.log("Public Key:", keypair.publicKey.toBase58());
		console.warn("Secret Key generated.", keypair.secretKey);
		return keypair;
	}
}
