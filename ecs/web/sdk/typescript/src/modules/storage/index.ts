import { Keypair, PublicKey } from "@solana/web3.js";
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
export class Solana implements solanaStorage {
  blueprint;
  program_id;
  signer;
  rpc_url;

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
  public migrate() {
    console.log(this.migrate);
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
  public set() {
    console.log("set method");
  }
}

function test() {
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
}

// ! WARNING: Test should not be in development environment
// ! Do it with build and start, not dev so the loop won't happen
// * uncomment if trying to tes
// test();
