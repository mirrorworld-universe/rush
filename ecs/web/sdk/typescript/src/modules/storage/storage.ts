import * as fs from "fs";
import * as path from "path";
import { IStorage } from "../../types/types";
import {
	Keypair,
	PublicKey,
	Connection,
	SystemProgram,
	Transaction,
	TransactionInstruction,
	sendAndConfirmTransaction,
} from "@solana/web3.js";

export class Storage {
	private blueprint;
	private programId;
	public signer;
	private rpcUrl;

	constructor({ blueprint, programId, signer, rpcUrl }: IStorage) {
		let blueprintPath = "";
		// TODO: initialize the blueprint
		let bluprint = (this.blueprint = blueprint);
		this.programId = programId;
		this.signer = signer;
		this.signer = signer;
		this.rpcUrl = rpcUrl;
	}

	/**
	 * migrate function
	 * /// TODO: DONE RULE: The Game Developer must be able to create an onchain world and
	 * spawn its initial entities based on the Rush Gaming Blueprint configuration
	 */
	public async migrate() {
		try {
			const connection = new Connection(this.rpcUrl);

			// Convert program_id to PublicKey if it's a string
			const programIdPubkey =
				typeof this.programId === "string"
					? new PublicKey(this.programId)
					: this.programId;

			// Get the world PDA (Program Derived Address)
			const [worldPDA] = PublicKey.findProgramAddressSync(
				[
					Buffer.from("world"),
					Buffer.from(this.blueprint),
					programIdPubkey.toBuffer(),
				],
				programIdPubkey,
			);

			// Create the instruction
			const instruction = new TransactionInstruction({
				programId: programIdPubkey,
				keys: [
					{
						pubkey: this.signer.publicKey,
						isSigner: true,
						isWritable: true,
					},
					{ pubkey: worldPDA, isSigner: false, isWritable: true },
					{
						pubkey: SystemProgram.programId,
						isSigner: false,
						isWritable: false,
					},
				],
				data: Buffer.from([
					/* instruction data */
				]),
			});

			const transaction = new Transaction().add(instruction);
			const signature = await sendAndConfirmTransaction(
				connection,
				transaction,
				[this.signer],
			);

			console.log("World created successfully. Signature:", signature);
			return signature;
		} catch (error) {
			console.error("Error in migrate:", error);
			throw error;
		}
	}

	/**
	 * create function
	 * /// TODO: DONE RULE: The Game Developer must be able to spawn an entity on the
	 * onchain game world in the Rush Store Solana Program (smart contract) after instantiating the SDK
	 */
	public create() {
		//? tasks breakdown:
		//? - learn how to create a transaction and storing instruction from the world detail
		//? - initiate Instance PDA Class
		//? - initiate World PDA Instance Class
		//? - create a transaction that will set all e2e
		console.log("create method");
	}

	/**
	 * delete function
	 */
	public delete() {
		console.log("delete method");
	}

	/**
	 * get function
	 * /// TODO: DONE RULE: The Game Developer must be able to retrieve specific entity
	 * data from their game’s On-chain world
	 */
	public get() {
		console.log("get method");
	}

	/**
	 * set function
	 * /// TODO: DONE RULE: The Game Developer must be able to update a specific entity data
	 * from their game’s Onchain world
	 */
	public async set(
		entityId: string,
		data: any,
		callback?: (signature: string) => void,
	) {
		console.log("set method");

		try {
			const connection = new Connection(this.rpcUrl);

			// Convert entityId to PublicKey
			const entityPubkey = new PublicKey(entityId);

			// Create the instruction
			const instruction = new TransactionInstruction({
				programId:
					this.programId instanceof PublicKey
						? this.programId
						: new PublicKey(this.programId),
				keys: [
					{
						pubkey: this.signer.publicKey,
						isSigner: true,
						isWritable: true,
					},
					{ pubkey: entityPubkey, isSigner: false, isWritable: true },
					{
						pubkey: SystemProgram.programId,
						isSigner: false,
						isWritable: false,
					},
				],
				data: Buffer.from(JSON.stringify(data)), // Assuming data is JSON serializable
			});

			const transaction = new Transaction().add(instruction);
			const signature = await sendAndConfirmTransaction(
				connection,
				transaction,
				[this.signer],
			);

			console.log("Entity updated successfully. Signature:", signature);
			return signature;
		} catch (error) {
			console.error("Error in set:", error);
			throw error;
		}
	}
}

async function test() {
	let Path = "";
	let PubKey = "";
	let keypair: Keypair;

	//! declare a keypair in a json file named <> with a publicKey and secretKey value pair

	// ? this one should be dynamic, auto generated after a sign-in of the wallet is engaged
	const KEYPAIR_PATH = path.join(__dirname, "SAMPLE_PAIR.json");
	Path = KEYPAIR_PATH;

	if (!fs.existsSync(KEYPAIR_PATH)) {
		keypair = Keypair.generate();
		const KEYPAIR_JSON = JSON.stringify({
			publicKey: keypair.publicKey.toString(),
			secretKey: Array.from(keypair.secretKey),
		});

		fs.writeFileSync(KEYPAIR_PATH, KEYPAIR_JSON);
		// notice
		console.log("New keypair generated and saved to", KEYPAIR_PATH);
	} else {
		const KEYPAIR_JSON = JSON.parse(fs.readFileSync(KEYPAIR_PATH, "utf-8"));
		const SECRET_KEY = Uint8Array.from(KEYPAIR_JSON.secretKey);
		keypair = Keypair.fromSecretKey(SECRET_KEY);
		PubKey = keypair.publicKey.toBase58();
		// notice
		console.log("Loaded keypair with public key: ", PubKey);
	}

	const programId = keypair.publicKey;

	const storage = new Storage({
		blueprint: "/path/to/blueprint",
		programId: programId.toString(),
		signer: keypair,
		rpcUrl: "http://127.0.0.1:8899",
	});
}

// ! WARNING: Test should not be in development environment
// ! Do it with build and start, not dev so the loop won't happen
// * uncomment if trying to test
// test();
