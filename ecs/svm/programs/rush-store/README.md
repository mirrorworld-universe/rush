# rush-store

> [!WARNING]
> Currently, account and data validations for CreateWorld haven't yet been implemented. Do not use in Mainnet production use until a official release occurs.

## Instructions Overview

- `CreateWorld`
- `UpdateWorld`
- `DeleteWorld`
- `SpawnEntity`
- `UpdateEntity`
- `DespawnEntity`

## CreateWorld

**`CreateWorld`** creates the World account onchain that is central to the Regions and Entities that facilitate your FOCGs/AWs onchain state.

## UpdateWorld

**`UpdateWorld`** creates the World account onchain that is central to the Regions and Entities that facilitate your FOCGs/AWs onchain state.

## DeleteWorld

**`DeleteWorld`** empties the account of its lamports into `world_authority` account (Signer) to remove rent-exemption and queue account for destruction. It also fills the account with 0s.

## SpawnEntity

> [!WARNING]
> Unimplemented

## UpdateEntity

> [!WARNING]
> Unimplemented

## DespawnEntity

> [!WARNING]
> Unimplemented
