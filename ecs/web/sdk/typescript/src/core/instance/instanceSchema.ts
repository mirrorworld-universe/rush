export class Instance {
	static TAG: string = "Instance";
}

export const InstanceSchema = new Map([
	[
		Instance,
		{
			kind: "struct",
			fields: [
				["discriminator", [32]],
				["name", "string"],
				["worldId", "string"],
				["ownerId", "string"],
				["createdAt", "u64"],
				["updatedAt", "u64"],
				["metadata", { kind: "map", key: "string", value: "string" }],
			],
		},
	],
]);
