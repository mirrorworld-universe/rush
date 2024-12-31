import { Keypair, PublicKey } from "@solana/web3.js";
import { ITsSdkParams } from "./core/types/types";
import { SolanaAdapters } from "./storage/SolanaAdapter";
import { StoragePort } from "./storage/StoragePort";

export type ComponentValue = string | number | boolean | BigInt;

export class RushSdk {
	private keypair: Keypair;
	private storage: StoragePort;

	constructor({ rpcUrl, programId, blueprintPath, keypair }: ITsSdkParams) {
		// Initialization of the instance
		this.keypair = keypair;
		const programIdKey = new PublicKey(programId);

		this.storage = new SolanaAdapters({
			blueprint: blueprintPath,
			programId: programIdKey,
			rpcUrl,
		});
	}
	//processComponent function to process the component
	private processComponent(entityId: string, component: ComponentValue) {
		try {
			if (
				typeof component !== "string" &&
				typeof component !== "number" &&
				typeof component !== "boolean" &&
				typeof component !== "bigint"
			) {
				throw new Error(
					`TypeMismatchError: Expected one of [string, number, boolean, BigInt] but received ${typeof component}`,
				);
			}

			console.log(
				`Processing component for entity ${entityId} with value: ${component}`,
			);

			// Store the component in the storage layer.
			return this.storage.set(entityId, component);
		} catch (error) {
			console.error("Error in processComponent:", error);
			throw error;
		}
	}

	public create() {
		this.storage.create();
	}

	public async migrate() {
		this.storage.migrate();
	}

	public async set(entityId: string, data: any) {
		// Set the entity
	}

	public async get(entityId: string) {
		// Get the entity
	}

	public async delete(entityId: string) {
		// Delete the entity
	}

	public signin(): Keypair {
		// Integrate RushSDK Session Auth?
		return Keypair.generate();
	}
}
