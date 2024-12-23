import {
	PublicKey,
	Keypair,
	Transaction,
	SystemProgram,
	LAMPORTS_PER_SOL,
	sendAndConfirmTransaction,
	Connection,
} from "@solana/web3.js";
import { Auth } from "./adapters/auth-adapter";
import crypto from "crypto";

export class SessionAuth implements Auth {
	// All variables can be stored in either PDA or just here in the class
	private connection: Connection; // Connection should be stored or they need to create a connection again
	private sessionExpiration: number | undefined; // Can store the session Expiration in PDA soon
	private userPublicKey: PublicKey | undefined = undefined; // This is the user public key Can be stored in PDA or localStorage

	constructor(connection: Connection) {
		this.connection = connection;
	}

	public async connectWallet(): Promise<PublicKey> {
		// Can add checker for exisiting wallet connection
		if (typeof window === "undefined") {
			throw new Error("This SDK must be used in a browser environment.");
		}

		if (!window.solana || !window.solana.isPhantom) {
			throw new Error("Phantom wallet is not installed.");
		}

		const response = await window.solana.connect();
		this.userPublicKey = response.publicKey;
		console.log("Connected with wallet:", response.publicKey.toBase58());
		return response.publicKey;
	}

	public createSession<T>(data: T): string {
		// Can add checker for exisiting session
		const newSession = Keypair.generate();
		const encryptedSession = this.encrypt(newSession, data);
		window.localStorage.setItem("sessionData", this.encryptData(data)); // encrypt the data and store it in the class (soon to be changed to use PDA)

		this.sessionExpiration = Date.now() + 60 * 60 * 1000; // Session expires in 1 hour

		window.localStorage.setItem("session", encryptedSession); // This will store the session in the browser
		return encryptedSession;
	}

	public async addFunds(
		amount: number,
		encryptedSessionKeypair: string,
	): Promise<void> {
		try {
			if (typeof window === "undefined") {
				throw new Error(
					"This SDK must be used in a browser environment.",
				);
			}

			if (!window.solana || !window.solana.isPhantom) {
				throw new Error("Phantom wallet is not installed.");
			}

			if (
				!window.localStorage.getItem("session") ||
				!window.localStorage.getItem("sessionData")
			) {
				throw new Error("No session found.");
			}

			const sessionKeypair = this.decrypt(encryptedSessionKeypair);

			const senderPublicKey = await this.connectWallet();
			const latestBlockhash = await this.connection.getLatestBlockhash();

			// Create transaction with recent blockhash
			const transaction = new Transaction({
				blockhash: latestBlockhash.blockhash,
				lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
				feePayer: senderPublicKey,
			}).add(
				SystemProgram.transfer({
					fromPubkey: senderPublicKey,
					toPubkey: sessionKeypair.publicKey,
					lamports: amount * LAMPORTS_PER_SOL,
				}),
			);

			// This will request the phantom since the main wallet of the user will sign for adding funds
			const { signature } =
				await window.solana.signAndSendTransaction(transaction);
			await this.connection.confirmTransaction(
				{
					signature,
					blockhash: latestBlockhash.blockhash,
					lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
				},
				"confirmed",
			);
			console.log(
				"Funds added successfully! Transaction Signature:",
				signature,
			);
		} catch (error) {
			throw new Error(error as unknown as string);
		}
	}

	public validateSession(encryptedSessionKeypair: string): boolean {
		const decryptedSessionKeypair = this.decrypt(encryptedSessionKeypair);

		// Check if session keypair exists and is not expired
		// Add more validation soon (Can store the session Expiration in PDA soon)
		if (
			!decryptedSessionKeypair ||
			!this.sessionExpiration ||
			Date.now() > this.sessionExpiration
		) {
			console.log("Session is invalid or expired.");
			return false;
		}

		console.log("Session is valid.");
		return true;
	}

	// Encrypt session
	private encrypt<T>(sessionKeypair: Keypair, data: T): string {
		console.log(data);
		const secretKey = crypto
			.createHash("sha256")
			.update(JSON.stringify(data))
			.digest();
		const iv = crypto.randomBytes(16);
		const serializedKeypair = JSON.stringify({
			publicKey: sessionKeypair.publicKey.toBase58(),
			secretKey: Buffer.from(sessionKeypair.secretKey).toString("base64"),
		});

		const cipher = crypto.createCipheriv("aes-256-cbc", secretKey, iv);
		let encrypted = cipher.update(serializedKeypair, "utf8", "base64");
		encrypted += cipher.final("base64");

		return iv.toString("hex") + encrypted;
	}

	// Decrypt the session
	private decrypt<T>(encryptedSessionKeypair: string): Keypair {
		if (!window.localStorage.getItem("sessionData")) {
			throw new Error("There is no sessionData");
		}
		const data = this.decryptData(
			window.localStorage.getItem("sessionData"),
		);
		const secretKey = crypto
			.createHash("sha256")
			.update(JSON.stringify(data))
			.digest();
		const iv = Buffer.from(encryptedSessionKeypair.substring(0, 32), "hex");
		const ciphertext = encryptedSessionKeypair.substring(32);

		const cipher = crypto.createDecipheriv("aes-256-cbc", secretKey, iv);
		let decrypted = cipher.update(ciphertext, "base64", "utf8");
		decrypted += cipher.final("utf8");

		const parsed = JSON.parse(decrypted);
		const keypairSecretKey = Uint8Array.from(
			Buffer.from(parsed.secretKey, "base64"),
		);

		return Keypair.fromSecretKey(keypairSecretKey);
	}

	// Encrypt the data the user use when initializing
	private encryptData<T>(data: T): string {
		const secretKey = crypto
			.createHash("sha256")
			.update("test-secretkey")
			.digest();
		const iv = crypto.randomBytes(16);

		const cipher = crypto.createCipheriv("aes-256-cbc", secretKey, iv);
		let encrypted = cipher.update(JSON.stringify(data), "utf8", "base64");
		encrypted += cipher.final("base64");
		console.log({ encryptedData: iv.toString("hex") + encrypted });
		return iv.toString("hex") + encrypted;
	}

	// Decrypt the data the user use when initializing
	private decryptData<T>(encryptedSessionData: string | undefined | null): T {
		console.log({ encryptedSessionData });
		if (!encryptedSessionData) {
			throw new Error("There is no session data");
		}
		const secretKey = crypto
			.createHash("sha256")
			.update("test-secretkey")
			.digest();
		const iv = Buffer.from(encryptedSessionData.substring(0, 32), "hex");
		const ciphertext = encryptedSessionData.substring(32);

		const cipher = crypto.createDecipheriv("aes-256-cbc", secretKey, iv);
		let decrypted = cipher.update(ciphertext, "base64", "utf8");
		decrypted += cipher.final("utf8");

		const parsed = JSON.parse(decrypted);

		return parsed;
	}

	public async refundFunds(
		sendTo: PublicKey | undefined,
		encryptedSessionKeypair: string,
	): Promise<void> {
		if (!sendTo) {
			throw new Error("There is no conencted wallet");
		}

		const sessionKeypair = this.decrypt(encryptedSessionKeypair);

		const balance = await this.connection.getBalance(
			sessionKeypair.publicKey,
		);

		if (balance === 0) {
			console.log("No funds to refund.");
			return;
		}

		const blockhash = await this.connection.getLatestBlockhash();

		// Initial Transaction
		const transaction = new Transaction({
			feePayer: sessionKeypair.publicKey,
			blockhash: blockhash.blockhash,
			lastValidBlockHeight: blockhash.lastValidBlockHeight,
		}).add(
			SystemProgram.transfer({
				fromPubkey: sessionKeypair.publicKey,
				toPubkey: sendTo,
				lamports: balance,
			}),
		);

		// Estimate fee before sending
		const feeCalculator = await this.connection.getFeeForMessage(
			transaction.compileMessage(),
		);
		const estimatedFee = feeCalculator.value || 0;
		console.log(`Estimated Fee: ${estimatedFee / LAMPORTS_PER_SOL} SOL`);

		// Check if balance is enough for the fee
		const netRefund = balance - estimatedFee;
		if (netRefund <= 0) {
			console.log("Insufficient funds after fees.");
			return;
		}

		// Update transaction with actual lamports after fee deduction
		transaction.instructions[0].data = Buffer.alloc(8);
		transaction.instructions[0] = SystemProgram.transfer({
			fromPubkey: sessionKeypair.publicKey,
			toPubkey: sendTo,
			lamports: netRefund,
		});

		// Send transaction using the session keys
		const signature = await sendAndConfirmTransaction(
			this.connection,
			transaction,
			[sessionKeypair],
		);

		console.log("Funds refunded successfully! Signature:", signature);
	}

	// This refund then revoke the session
	public revokeSession(encryptedSessionKeypair: string): void {
		if (!window) {
			throw new Error("Need to be on window to revoke");
		}
		// Basically refunds the funds first then removes everything on the session
		this.refundFunds(this.userPublicKey, encryptedSessionKeypair);
		console.log("Session revoked.");
		window.localStorage.removeItem("sessionData");
		window.localStorage.removeItem("session");
	}

	// This will be the transaction creation using the session keys
	public async createTransaction(
		amount: number,
		encryptedSessionKeypair: string,
		recipient: PublicKey,
	): Promise<void> {
		if (typeof window === "undefined") {
			throw new Error("This SDK must be used in a browser environment.");
		}

		if (!window.solana || !window.solana.isPhantom) {
			throw new Error("Phantom wallet is not installed.");
		}

		const blockhash = await this.connection.getLatestBlockhash();

		const sessionKeypair = this.decrypt(encryptedSessionKeypair);

		const transaction = new Transaction({
			feePayer: sessionKeypair.publicKey,
			blockhash: blockhash.blockhash,
			lastValidBlockHeight: blockhash.lastValidBlockHeight,
		}).add(
			SystemProgram.transfer({
				fromPubkey: sessionKeypair.publicKey,
				toPubkey: recipient,
				lamports: amount * LAMPORTS_PER_SOL,
			}),
		);

		const signature = await sendAndConfirmTransaction(
			this.connection,
			transaction,
			[sessionKeypair],
		);

		await this.connection.confirmTransaction(
			{
				signature,
				blockhash: blockhash.blockhash,
				lastValidBlockHeight: blockhash.lastValidBlockHeight,
			},
			"confirmed",
		);
		console.log("Transaction created successfully! Signature:", signature);
	}
}
