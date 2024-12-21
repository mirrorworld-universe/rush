import bs58 from "bs58";
import { Storage } from "./modules/storage/storage";
import { RushSdk } from "./sdk";
import { Keypair, PublicKey } from "@solana/web3.js";
console.log(
	"seeing this means you have successfully ran the scripts \nfrom src/test.ts file\n",
);

async function test_call_storage() {
	const new_keypair = Keypair.generate();

	const storage = new Storage({
		blueprint: "/my/blueprint/path",
		programId: new PublicKey(
			"6vg3oUN7LLcCS3Qc8bhsrqqJRkeDaC2KsFqF23aQp5iQ",
		),
		signer: new_keypair,
		rpcUrl: "http://127.0.0.1:8899",
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
		console.log(
			"Set function executed successfully. Signature:",
			signature,
		);
	} catch (error) {
		console.error("Error executing set function:", error);
	}
}

function test_call_rushsdk() {
	const new_keypair = Keypair.generate();
	const encoded = bs58.encode(new_keypair.secretKey);
	const secretKey = bs58.decode(encoded); // Pretend to be a secret key to be passed to creating the keypair
	const sdk = new RushSdk({
		secretKey: secretKey,
		blueprintPath: "/my/blueprint/path",
		programId: new_keypair.publicKey,
		rpcUrl: "http://127.0.0.1:8899",
	});

	console.log(sdk);
}

function test_call_migrate() {
	const new_keypair = Keypair.generate();
	const encoded = bs58.encode(new_keypair.secretKey);
	const secretKey = bs58.decode(encoded); // Pretend to be a secret key to be passed to creating the keypair
	const sdk = new RushSdk({
		secretKey: secretKey,
		blueprintPath: "/my/blueprint/path",
		programId: new_keypair.publicKey,
		rpcUrl: "http://127.0.0.1:8899",
	});
	sdk.migrate();
	// console.log(sdk);
}

function test_call_create() {
	const new_keypair = Keypair.generate();
	const encoded = bs58.encode(new_keypair.secretKey);
	const secretKey = bs58.decode(encoded); // Pretend to be a secret key to be passed to creating the keypair
	
	const sdk = new RushSdk({
		secretKey: secretKey,
		blueprintPath: "/my/blueprint/path",
		programId: new_keypair.publicKey,
		rpcUrl: "http://127.0.0.1:8899",
	});

	sdk.create();
}

async function setStorage(entityId: string, data: object) {
	const new_keypair = Keypair.generate();

	const storage = new Storage({
		blueprint: "/my/blueprint/path",
		programId: new PublicKey(
			"6vg3oUN7LLcCS3Qc8bhsrqqJRkeDaC2KsFqF23aQp5iQ",
		),
		signer: new_keypair,
		rpcUrl: "http://127.0.0.1:8899",
	});

	try {
		const signature = await storage.set(entityId, data);
		console.log(
			"Set function executed successfully. Signature:",
			signature,
		);
	} catch (error) {
		console.error("Error executing set function:", error);
	}
}

// setStorage("your_entity_id_here", { key: "value" }); // Sample call to setStorage
// test_call_rushsdk();
// test_call_migrate();
// test_call_storage();
test_call_create();
