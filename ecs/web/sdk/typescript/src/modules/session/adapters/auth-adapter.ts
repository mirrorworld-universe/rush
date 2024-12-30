import { Keypair, PublicKey } from "@solana/web3.js";

export interface Auth {
	connectWallet(): Promise<PublicKey>;

	// This will create the session keypair and encrypt the data and then store that encrypted data to the class and return the encrypted session keypair (base64 string)
	createSession<T>(data: T): string;

	// Call this function to add funds to the session
	addFunds(
		amount: number,
		encryptedSessionKeypair: string,
		senderPublickey: PublicKey,
	): void;

	// This will be the validator for overall session like the funds, expiration
	validateSession(encryptedSessionKeypair: string): boolean;

	// This will encrypt the keypair using the data the user passed when creating a session to be used for creating a hash for encryption.
	// encrypt<T>(sessionKeypair: Keypair, data: T): string;

	// This will decrypt the encrypted sessionKeypair which is in a base64 string and return it as Keypair
	// decrypt<T>(encryptedSessionKeypair: string, data: T): Keypair;

	// Refunds the fund stored in the session to the users wallet
	refundFunds(
		sendTo: PublicKey,
		encryptedSessionKeypair: string,
	): Promise<void>;

	// This will call the refundFunds function to return the funds stored on the session and then revoke the session
	revokeSession(encryptedSessionKeypair: string): void;

	// This will then be used for every transaction when a session is already been made
	createTransaction(
		amount: number,
		encryptedSessionKeypair: string,
		recipient: PublicKey,
	): Promise<void>;
}
