import { PublicKey } from "@solana/web3.js";

// Possible to make this a class?
interface Entity {
	name: string;
	description: string;
	properties: Map<string, string>;
}

interface Region {
	name: string;
	description: string;
	properties: Map<string, string>;
}

export class World {
	discriminator: Uint8Array;
	name: string;
	description: string;
	entities: Entity[];
	regions: Region[];
	instances: Map<Region, Map<Entity, number>>;
	isLaunched: boolean;
	worldAuthority: PublicKey;
	bump: number;

	TAG: string = "World";

	constructor(
		discriminator: Uint8Array,
		name: string,
		description: string,
		entities: Entity[],
		regions: Region[],
		instances: Map<Region, Map<Entity, number>>,
		isLaunched: boolean,
		worldAuthority: PublicKey,
		bump: number,
	) {
		this.discriminator = discriminator;
		this.name = name;
		this.description = description;
		this.entities = entities;
		this.regions = regions;
		this.instances = instances;
		this.isLaunched = isLaunched;
		this.worldAuthority = worldAuthority;
		this.bump = bump;
	}

	findPda(
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

	createPda(
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
