import { PublicKey } from "@solana/web3.js";

export class World {
	static TAG: string = "World";

	constructor() {}

	static findPda(
		program_id: PublicKey,
		name: string,
		description: string,
	): { pda: PublicKey; bump: number } {
		const seed = [
			Buffer.from(this.TAG),
			Buffer.from(name),
			Buffer.from(description),
		];
		const [pda, bump] = PublicKey.findProgramAddressSync(seed, program_id);
		return { pda, bump };
	}

	static createPda(
		programId: PublicKey,
		name: string,
		description: string,
		worldAuthority: PublicKey,
		bumpSeed: number,
	): PublicKey {
		const seed = [
			Buffer.from(this.TAG),
			Buffer.from(name),
			Buffer.from(description),
			worldAuthority.toBuffer(),
			Buffer.from([bumpSeed]),
		];
		const pda = PublicKey.createProgramAddressSync(seed, programId);
		return pda;
	}
}
