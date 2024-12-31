"use client";

import React, { useState } from "react";
import { RushSession } from "../../../../sdk/typescript/src/session/RushSessionAdapter";
import { Connection, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";

const connection = new Connection("https://api.devnet.solana.com", "confirmed");
const sessionAuth = new RushSession(connection);
const SessionTest = () => {
	const [walletPublicKey, setWalletPublicKey] = useState<PublicKey | null>(null);
	const [encryptedSession, setEncryptedSession] = useState<string | null>(window.localStorage.getItem("session"));
	const [transactionAmount, setTransactionAmount] = useState<number>(0);

	// Connect Wallet
	const connectWallet = async () => {
		try {
			const publicKey = await sessionAuth.connectWallet();
			setWalletPublicKey(publicKey);
			console.log("Wallet Public Key:", publicKey.toBase58());
		} catch (error) {
			console.error("Error connecting wallet:", error);
		}
	};

	// Create Session
	const createSession = () => {
		try {
			const session = sessionAuth.createSession({ user: "Alice" });
			setEncryptedSession(session);
			console.log("Encrypted Session:", session);
		} catch (error) {
			console.error("Error creating session:", error);
		}
	};

	// Add Funds
	const addFunds = async () => {
		try {
			if (!encryptedSession) throw new Error("Session not created!");
			await sessionAuth.addFunds(0.1, encryptedSession);
			console.log("Funds added successfully!");
		} catch (error: any) {
			console.error("Error adding funds:", error?.message as unknown as string);
		}
	};

	// Create Transaction
	const createTransaction = async (amount: number) => {
		try {
			if (!encryptedSession || !walletPublicKey) throw new Error("Missing session or wallet key!");
			await sessionAuth.createTransaction(amount, encryptedSession, walletPublicKey);
			console.log("Transaction completed!");
		} catch (error) {
			console.error("Error creating transaction:", error);
		}
	};

	// Refund Funds
	const refundFunds = async () => {
		try {
			if (!encryptedSession || !walletPublicKey) throw new Error("Missing session or wallet key!");
			await sessionAuth.refundFunds(walletPublicKey, encryptedSession);
			console.log("Refund completed!");
		} catch (error) {
			console.error("Error refunding funds:", error);
		}
	};

	// Validate Session
	const validateSession = () => {
		try {
			if (!encryptedSession) throw new Error("Session not created!");
			const isValid = sessionAuth.validateSession(encryptedSession);
			console.log("Session Valid:", isValid);
		} catch (error) {
			console.error("Error validating session:", error);
		}
	};

	// Revoke Session
	const revokeSession = () => {
		try {
			if (!encryptedSession) throw new Error("Session not created!");
			const isValid = sessionAuth.revokeSession(encryptedSession);
			console.log("Session Valid:", isValid);
		} catch (error) {
			console.error("Error validating session:", error);
		}
	};

	return (
		<div>
			<h1>SessionAuth SDK Test</h1>
			<div className="flex space-x-2">
				<button
					className="border-2 border-solid"
					onClick={connectWallet}>
					Connect Wallet
				</button>
				<button
					className="border-solid border-2"
					onClick={createSession}>
					Create Session
				</button>
				<button
					className="border-2 border-solid"
					onClick={addFunds}>
					Add Funds
				</button>
				<button
					className="border-2 border-solid"
					onClick={refundFunds}>
					Refund Funds
				</button>
				<button
					className="border border-solid"
					onClick={validateSession}>
					Validate Session
				</button>
				<button
					className="border border-solid"
					onClick={revokeSession}>
					Revoke Session
				</button>
			</div>
			<div className="flex flex-col mt-2 w-32">
				<input
					type="number"
					value={transactionAmount}
					onChange={(e) => setTransactionAmount(e.target.valueAsNumber)}
					className="border-2 border-solid"
				/>
				<button
					className="border-2 border-solid"
					onClick={(e) => {
						e.preventDefault();
						createTransaction(transactionAmount);
					}}>
					Create Transaction
				</button>
			</div>
		</div>
	);
};

export default SessionTest;
