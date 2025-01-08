import { PublicKey } from "@solana/web3.js";
import { Entity, Region } from "../types/Types";

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
}
