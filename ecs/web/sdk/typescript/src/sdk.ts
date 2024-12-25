import { Keypair, PublicKey } from "@solana/web3.js";
import { Storage } from "./modules/storage/storage";
import { ITsSdkParams } from "./types/types";

export type ComponentValue = string | number | boolean | BigInt;

export class RushSdk {
  private keypair: Keypair;
  private storage: Storage;

  constructor({ rpcUrl, programId, blueprintPath, keypair }: ITsSdkParams) {
    // Initialization of the instance
    this.keypair = keypair;
    const programIdKey = new PublicKey(programId);

    this.storage = new Storage({
      blueprint: blueprintPath,
      programId: programIdKey,
      rpcUrl,
      signer: keypair,
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
          `TypeMismatchError: Expected one of [string, number, boolean, BigInt] but received ${typeof component}`
        );
      }

      console.log(
        `Processing component for entity ${entityId} with value: ${component}`
      );

      // Store the component in the storage layer.
      return this.storage.set(entityId, component);
    } catch (error) {
      console.error("Error in processComponent:", error);
      throw error;
    }
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

  public async migrate() {
    try {
      const signature = await this.storage.migrate();
      console.log("Migration successful. Signature:", signature);
      return signature;
    } catch (error) {
      console.error("Error during migration:", error);
      throw error;
    }
  }

  public signin(): Keypair {
    const keypair = Keypair.generate();
    console.log("New Keypair Created:");
    console.log("Public Key:", keypair.publicKey.toBase58());
    console.warn("Secret Key generated.", keypair.secretKey);
    return keypair;
  }
}
