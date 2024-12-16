import { Keypair, PublicKey } from "@solana/web3.js";
import { Solana } from "./modules/storage";

// ? this is a mock test
// ? this is where the solana storage class is called inside the TS_RUSH_SDK

function test_call_storage() {
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
}

// test_call_storage();


console.log('sdk index file')