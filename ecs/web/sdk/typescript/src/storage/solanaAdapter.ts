import {
	PublicKey,
	Keypair,
	Connection,
	Transaction,
	TransactionInstruction,
} from "@solana/web3.js";
import { Storage } from "../core/port/StoragePort";

// Define InstructionData class
export class InstructionData { // Added export here
  public property1: string;
  public property2: number;

  constructor(property1: string, property2: number) {
    this.property1 = property1;
    this.property2 = property2;
  }

  // Borsh serialization method
  static borshSerialize(data: InstructionData): Buffer {
    const property1Buffer = Buffer.from(data.property1, 'utf-8');
    const property1Length = Buffer.alloc(4);
    property1Length.writeUInt32LE(property1Buffer.length, 0);

    const property2Buffer = Buffer.alloc(4);
    property2Buffer.writeUInt32LE(data.property2, 0);

    const buffer = Buffer.concat([property1Length, property1Buffer, property2Buffer]);
    return buffer;
  }

  // Borsh deserialization method
  static borshDeserialize(buffer: Buffer): InstructionData {
    const property1Length = buffer.readUInt32LE(0);
    const property1 = buffer.toString('utf-8', 4, 4 + property1Length);
    const property2 = buffer.readUInt32LE(4 + property1Length);

    return new InstructionData(property1, property2);
  }

  // Serialize method
  serialize(): Buffer {
    return InstructionData.borshSerialize(this);
  }
}

// Define a function to create a transaction instruction
function createTransactionInstruction(
  worldPDA: PublicKey,
  sessionKeypair: Keypair,
  programId: PublicKey,
  data: InstructionData
): TransactionInstruction {
  return new TransactionInstruction({
    keys: [
      { pubkey: worldPDA, isSigner: false, isWritable: true },
      { pubkey: sessionKeypair.publicKey, isSigner: true, isWritable: true },
    ],
    programId,
    data: data.serialize(),
  });
}

// Define a function to send a transaction
async function sendTransaction(
  connection: Connection,
  transaction: Transaction,
  sessionKeypair: Keypair
): Promise<string> {
  const signature = await connection.sendTransaction(transaction, [sessionKeypair]);
  await connection.confirmTransaction(signature);
  return signature;
}

export class SolanaAdapters implements Storage {
  private blueprint: string = "";
  private programId: PublicKey;
  private sessionKeypair: Keypair = Keypair.generate();
  private rpcUrl: string = "";
  private connection: Connection;

  constructor({
    blueprint,
    programId,
    rpcUrl,
  }: {
    blueprint: string;
    programId: PublicKey;
    rpcUrl?: string;
  }) {
    this.blueprint = blueprint;
    this.programId = new PublicKey(programId);
    this.rpcUrl = rpcUrl || blueprint;
    this.connection = new Connection(this.rpcUrl);
  }

	public getProgramId(): PublicKey {
		return this.programId;
	}

	public getRpcUrl(): string {
		return this.rpcUrl;
	}

	private async getWorldPDA(): Promise<PublicKey> {
    return (await PublicKey.findProgramAddress([Buffer.from("world")], this.programId))[0];
  }

  public async migrate(): Promise<void> {
    try {
      const worldPDA = await this.getWorldPDA();
      const transaction = new Transaction();
      const instruction = createTransactionInstruction(
        worldPDA,
        this.sessionKeypair,
        this.programId,
        new InstructionData("example", 123)
      );
      transaction.add(instruction);
      const signature = await sendTransaction(this.connection, transaction, this.sessionKeypair);
      console.log("Entities spawned with transaction signature:", signature);
    } catch (error) {
        console.error("Error during migration:", error);
        throw error; // Rethrow the error to ensure the promise rejects
    }
  }

  public async create(): Promise<void> {
    try {
      const worldPDA = await this.getWorldPDA();
      const transaction = new Transaction();
      const instruction = createTransactionInstruction(
        worldPDA,
        this.sessionKeypair,
        this.programId,
        new InstructionData("newEntity", 456)
      );
      transaction.add(instruction);
      const signature = await sendTransaction(this.connection, transaction, this.sessionKeypair);
      console.log("New entity created with transaction signature:", signature);
    } catch (error) {
        console.error("Error during creation:", error);
        throw error; // Rethrow the error to ensure the promise rejects
    }
  }

  public async get(entityId: string): Promise<void> {
    try {
      const worldPDA = await this.getWorldPDA();
      const transaction = new Transaction();
      const instruction = createTransactionInstruction(
        worldPDA,
        this.sessionKeypair,
        this.programId,
        new InstructionData(entityId, 789) // Using entityId
      );
      transaction.add(instruction);
      const signature = await sendTransaction(this.connection, transaction, this.sessionKeypair);
      console.log("Entity retrieved with transaction signature:", signature);
    } catch (error) {
      console.error("Error during retrieval:", error);
      throw error; // Rethrow the error to ensure the promise rejects
      throw error; // Rethrow the error to ensure the promise rejects
    }
  }

  public async set(entityId: string, data: { property1: string; property2: number }): Promise<void> {
    try {
      const worldPDA = await this.getWorldPDA();
      const transaction = new Transaction();
      const instruction = createTransactionInstruction(
        worldPDA,
        this.sessionKeypair,
        this.programId,
        new InstructionData(data.property1, data.property2)
      );
      transaction.add(instruction);
      const signature = await sendTransaction(this.connection, transaction, this.sessionKeypair);
      console.log("Entity updated with transaction signature:", signature);
    } catch (error) {
        console.error("Error during update:", error);
        throw error; // Rethrow the error to ensure the promise rejects
    }
  }

  public getSchema(): object {
    return {
      properties: {
        property1: { type: "string" },
        property2: { type: "number" },
      },
      required: ["property1", "property2"],
    };
  }

  public async delete(entityId: string): Promise<void> {
    try {
      const worldPDA = await this.getWorldPDA();
      const transaction = new Transaction();
      const instruction = createTransactionInstruction(
        worldPDA,
        this.sessionKeypair,
        this.programId,
        new InstructionData(entityId, 0) // Using entityId
      );
      transaction.add(instruction);
      const signature = await sendTransaction(this.connection, transaction, this.sessionKeypair);
      console.log("Entity deleted with transaction signature:", signature);
    } catch (error) {
        console.error("Error during deletion:", error);
        throw error; // Rethrow the error to ensure the promise rejects
    }
  }
}
