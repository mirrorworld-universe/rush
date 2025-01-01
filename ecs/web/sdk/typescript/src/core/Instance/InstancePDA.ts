import { PublicKey } from "@solana/web3.js";

class InstancePDA {
	static TAG: string = "Instance";

	static findPda(
		programId: PublicKey,
		worldPda: PublicKey,
		region: string,
		entity: string,
		nonce: BigUint64Array,
	): [PublicKey, number] {
		return PublicKey.findProgramAddressSync(
			[
				Buffer.from(InstancePDA.TAG),
				worldPda.toBuffer(),
				Buffer.from(region),
				Buffer.from(entity),
				Buffer.from(new Uint8Array(nonce)),
			],
			programId,
		);
	}

	static createPda(
		programId: PublicKey,
		worldPda: PublicKey,
		region: string,
		entity: string,
		nonce: BigUint64Array,
		bumpSeed: number,
	): PublicKey {
		return PublicKey.createProgramAddressSync(
			[
				Buffer.from(InstancePDA.TAG),
				worldPda.toBuffer(),
				Buffer.from(region),
				Buffer.from(entity),
				Buffer.from(new Uint8Array(nonce)),
				Buffer.from([bumpSeed]),
			],
			programId,
		);
	}
}

export default InstancePDA;
