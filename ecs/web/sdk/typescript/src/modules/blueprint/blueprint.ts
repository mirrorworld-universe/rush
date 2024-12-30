//? this is a placeholder class for BtreeMap, module to be developed? or is it a library?

import { IBluePrint, IBTreeMap } from "../../types/types";

class BTreeMap implements IBTreeMap {
	//! should use collections library for this

	K;
	V;
	A;

	constructor(K: string = "", V: string = "", A: string = "") {
		this.K = K;
		this.V = V;
		this.A = A;
	}
}

export class BluePrint implements IBluePrint {
	name;
	description;
	entities;
	regions;
	instances;
	// description: world_description,
	// entities: BTreeMap::new(),
	// regions: BTreeMap::new(),
	// instances: BTreeMap::new(),

	constructor(
		name: string,
		description: string,
		entities: BTreeMap = new BTreeMap(),
		regions: BTreeMap = new BTreeMap(),
		instances: BTreeMap = new BTreeMap(),
	) {
		this.name = name;
		this.description = description;
		this.entities = entities;
		this.regions = regions;
		this.instances = instances;
	}
}
