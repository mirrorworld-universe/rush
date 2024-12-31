import { World } from "./WorldPDA";

export const WorldSchema = new Map([
	[
		World,
		{
			kind: "struct",
			fields: [
				["discriminator", [32]],
				["name", "string"],
				["description", "string"],
				["entities", ["Entity"]],
				["regions", ["Region"]],
				["instances", [Map, ["Region", [Map, ["Entity", "u64"]]]]],
				["isLaunched", "bool"],
				["worldAuthority", "publicKey"],
				["bump", "u8"],
			],
		},
	],
]);
