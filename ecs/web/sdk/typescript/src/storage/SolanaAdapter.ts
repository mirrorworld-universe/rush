import {
	PublicKey,
	Keypair,
	Connection,
	Transaction,
	TransactionInstruction,
} from "@solana/web3.js";
import { Entity } from "../core/types/types";
import { StoragePort } from "./StoragePort";

export class SolanaAdapters implements StoragePort {
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

	public async migrate(): Promise<void> {
		// Migration logic
	}

	public async create(): Promise<void> {
		// Create the entity
	}

	public async get() {
		// Gets The entity
	}

	public async set() {
		// Sets the entity
	}

	public async delete() {
		// Deletes the entity
	}
}
