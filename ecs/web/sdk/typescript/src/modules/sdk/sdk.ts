import { TSSDKParams } from "../../types";
import { Solana } from "../storage";
import { Keypair, PublicKey } from "@solana/web3.js";
import bs58 from "bs58";

/**
 * RushSDK class that provides SDK functionality for interacting with Solana.
 */
export class RushSDK {
  // Important variables
  private keypair: Keypair;
  private storage: Solana;

  constructor({ rpc_url, program_id, blueprint_path, secret_key }: TSSDKParams) {
    // Initialization of the instance
    this.keypair = Keypair.fromSecretKey(secret_key); // Create a key pair using the provided secret key

    const programIdKey = new PublicKey(program_id);

		this.storage = new Solana({ blueprint: blueprint_path, program_id: programIdKey, rpc_url, signer: this.keypair });
	}

	/**
	 * Public set function to update entity data in the on-chain world.
	 * @param entityId - The ID of the entity to update.
	 * @param data - The data to update for the entity.
	 */
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
  
	public Migrate() {
		// Migrate the data
		this.storage.migrate();
	}


  /**
   * Generates a new Solana Keypair and returns it.
   * @returns {Keypair} A new Solana Keypair.
   */
  private Signin(): Keypair {
    const keypair = Keypair.generate();
    console.log("New Keypair Created:");
    console.log("Public Key:", keypair.publicKey.toBase58());
    console.warn("Secret Key generated.", keypair.secretKey);
    return keypair;
  }
}
