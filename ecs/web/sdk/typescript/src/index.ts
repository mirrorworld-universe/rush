import { Keypair, PublicKey } from "@solana/web3.js";
import { Solana } from "./modules/storage";
import { RushSDK } from "./modules/sdk/sdk";
import bs58 from "bs58";

// ? this is a mock test
// ? this is where the solana storage class is called inside the TS_RUSH_SDK

async function test_call_storage() {
	const new_keypair = Keypair.generate();

	const storage = new Solana({
		blueprint: "/my/blueprint/path",
		program_id: new PublicKey("6vg3oUN7LLcCS3Qc8bhsrqqJRkeDaC2KsFqF23aQp5iQ"),
		signer: new_keypair,
		rpc_url: "http://127.0.0.1:8899",
	});

	console.log(storage);
	console.log("signer :", {
		"pubkey :": storage.signer.publicKey,
		"sec key :": storage.signer.secretKey,
	});
	console.log(storage.signer.secretKey);

	// Call the set method
	const entityId = "your_entity_id_here"; // Replace with a valid entity ID
	const data = { key: "value" }; // Sample data to update

	try {
		const signature = await storage.set(entityId, data);
		console.log("Set function executed successfully. Signature:", signature);
	} catch (error) {
		console.error("Error executing set function:", error);
	}
}

function test_call_rushsdk() {
	const new_keypair = Keypair.generate();
	const encoded = bs58.encode(new_keypair.secretKey);
	const secretKey = bs58.decode(encoded); // Pretend to be a secret key to be passed to creating the keypair
	const sdk = new RushSDK({
		secret_key: secretKey,
		blueprint_path: "/my/blueprint/path",
		program_id: new_keypair.publicKey,
		rpc_url: "http://127.0.0.1:8899",
	});

	console.log(sdk);
}

// test_call_rushsdk();
console.log("sdk index file");

setStorage("your_entity_id_here", { key: "value" }); // Sample call to setStorage

async function setStorage(entityId: string, data: object) {
	const new_keypair = Keypair.generate();

	const storage = new Solana({
		blueprint: "/my/blueprint/path",
		program_id: new PublicKey("6vg3oUN7LLcCS3Qc8bhsrqqJRkeDaC2KsFqF23aQp5iQ"),
		signer: new_keypair,
		rpc_url: "http://127.0.0.1:8899",
	});

	try {
		const signature = await storage.set(entityId, data);
		console.log("Set function executed successfully. Signature:", signature);
	} catch (error) {
		console.error("Error executing set function:", error);
	}
}
