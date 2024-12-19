import { Keypair, PublicKey } from "@solana/web3.js";
import { Storage } from "./modules/storage/storage";
import { ITsSdkParams } from "./types/types";

/**
 * RushSDK class that provides SDK functionality for interacting with Solana.
 */
export class RushSdk {
	// Important variables
	private keypair: Keypair;
	private storage: Storage;

	constructor({ rpcUrl, programId, blueprintPath, secretKey }: ITsSdkParams) {
		// Initialization of the instance
		this.keypair = Keypair.fromSecretKey(secretKey); // Create a key pair using the provided secret key

		const programIdKey = new PublicKey(programId);

		this.storage = new Storage({
			blueprint: blueprintPath,
			programId: programIdKey,
			rpcUrl,
			signer: this.keypair,
		});
	}

	/**
	 * Public set function to update entity data in the on-chain world.
	 * @param entityId - The ID of the entity to update.
	 * @param data - The data to update for the entity.
	 */
	public async set(entityId: string, data: any) {
		try {
			const signature = await this.storage.set(entityId, data);
			console.log(
				"Set function executed successfully. Signature:",
				signature,
			);
			return signature;
		} catch (error) {
			console.error("Error in set function:", error);
			throw error;
		}
	}

	public create() {
		// Migrate the data
		this.storage.create();
	}

	public migrate() {
		// Migrate the data
		this.storage.migrate();
	}

	/**
	 * Generates a new Solana Keypair and returns it.
	 * @returns {Keypair} A new Solana Keypair.
	 */
	public signin(): Keypair {
		const keypair = Keypair.generate();
		console.log("New Keypair Created:");
		console.log("Public Key:", keypair.publicKey.toBase58());
		console.warn("Secret Key generated.", keypair.secretKey);
		return keypair;
	}
}
