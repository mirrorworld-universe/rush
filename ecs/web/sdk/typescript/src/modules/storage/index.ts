import { Keypair, PublicKey, Connection, SystemProgram, Transaction, TransactionInstruction, sendAndConfirmTransaction } from "@solana/web3.js";
import { solanaStorage } from "../../types";
import * as fs from "fs";
import * as path from "path";


// ! in this solana

// ? use solana client : rpc client, rpc client
// !

// ! solana sdk :
// ? pubkey,
// ? signer: { keypair, keypair, signer }
// ? transaction

// ! impl solana {
// !  pub fn new(progam id: pubkey, signer: keypair, rpc_url: string, path: )
// ! }
export class Solana {
	private blueprint;
	private program_id;
	public signer;
	private rpc_url;

	constructor({ blueprint, program_id, signer, rpc_url }: solanaStorage) {
		let Path = "";

		let bluprint = (this.blueprint = blueprint);
		this.program_id = program_id;
		this.signer = signer;
		this.signer = signer;
		this.rpc_url = rpc_url;
	}

	/**
	 * migrate function
	 * /// TODO: DONE RULE: The Game Developer must be able to create an onchain world and
	 * spawn its initial entities based on the Rush Gaming Blueprint configuration
	 */
	public async migrate() {
		try {
			const connection = new Connection(this.rpc_url);
			
			// Convert program_id to PublicKey if it's a string
			const programIdPubkey = (typeof this.program_id === 'string') 
				? new PublicKey(this.program_id) 
				: this.program_id;

			// Get the world PDA (Program Derived Address)
			const [worldPDA] = PublicKey.findProgramAddressSync(
				[
					Buffer.from("world"),
					Buffer.from(this.blueprint),
					programIdPubkey.toBuffer()
				],
				programIdPubkey
			);

			// Create the instruction
			const instruction = new TransactionInstruction({
				programId: programIdPubkey,
				keys: [
					{ pubkey: this.signer.publicKey, isSigner: true, isWritable: true },
					{ pubkey: worldPDA, isSigner: false, isWritable: true },
					{ pubkey: SystemProgram.programId, isSigner: false, isWritable: false }
				],
				data: Buffer.from([/* instruction data */])
			});

			const transaction = new Transaction().add(instruction);
			const signature = await sendAndConfirmTransaction(
				connection,
				transaction,
				[this.signer]
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
public async set(entityId: string, data: any, callback?: (signature: string) => void) {
	    console.log("set method");

	    try {
	        const connection = new Connection(this.rpc_url);
	        
	        // Convert entityId to PublicKey
	        const entityPubkey = new PublicKey(entityId);

	        // Create the instruction
	        const instruction = new TransactionInstruction({
	            programId: this.program_id instanceof PublicKey ? this.program_id : new PublicKey(this.program_id),
	            keys: [
	                { pubkey: this.signer.publicKey, isSigner: true, isWritable: true },
	                { pubkey: entityPubkey, isSigner: false, isWritable: true },
	                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }
	            ],
	            data: Buffer.from(JSON.stringify(data)) // Assuming data is JSON serializable
	        });

	        const transaction = new Transaction().add(instruction);
	        const signature = await sendAndConfirmTransaction(
	            connection,
	            transaction,
	            [this.signer]
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
    const storage = new Solana({
        blueprint: "/path/to/blueprint",
        program_id: program_id.toString(),
        signer: KEYPAIR,
        rpc_url: "http://127.0.0.1:8899",
    });

    // Test the set function
    const entityId = "your_entity_id_here"; // Replace with a valid entity ID
    const data = { key: "value" }; // Sample data to update

    try {
        const signature = await storage.set(entityId, data);
        console.log("Set function executed successfully. Signature:", signature);
    } catch (error) {
        console.error("Error executing set function:", error);
    }
}

// Uncomment to run the test
// test();

// Uncomment to run the test
// test();

// Uncomment to run the test
// test();

// Uncomment to run the test
// test();
	let Path = "";
	let PubKey = "";
	let KEYPAIR: Keypair;
	let KEYPAIR_JSON;

	//! declare a keypair in a json file named <> with a publicKey and secretKey value pair

	// ? this one should be dynamic, auto generated after a sign-in of the wallet is engaged
	const KEYPAIR_PATH = path.join(__dirname, "SAMPLE_PAIR.json");
	Path = KEYPAIR_PATH;

	if (!fs.existsSync(KEYPAIR_PATH)) {
		KEYPAIR = Keypair.generate();
		const KEYPAIR_JSON = JSON.stringify({
			publicKey: KEYPAIR.publicKey.toString(),
			secretKey: Array.from(KEYPAIR.secretKey),
		});

		fs.writeFileSync(KEYPAIR_PATH, KEYPAIR_JSON);
		// notice
		console.log("New keypair generated and saved to", KEYPAIR_PATH);
	} else {
		const KEYPAIR_JSON = JSON.parse(fs.readFileSync(KEYPAIR_PATH, "utf-8"));
		const SECRET_KEY = Uint8Array.from(KEYPAIR_JSON.secretKey);
		KEYPAIR = Keypair.fromSecretKey(SECRET_KEY);
		PubKey = KEYPAIR.publicKey.toBase58();
		// notice
		console.log("Loaded keypair with public key:", PubKey);
	}

	// let program_id = new PublicKey(PubKey);
	const program_id = KEYPAIR.publicKey;

	const storage = new Solana({
		blueprint: "/path/to/blueprint",
		program_id: program_id.toString(),
		signer: KEYPAIR,
		rpc_url: "http://127.0.0.1:8899",
	});


// ! WARNING: Test should not be in development environment
// ! Do it with build and start, not dev so the loop won't happen
// * uncomment if trying to tes
// test();
