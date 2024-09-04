# rush-store

> [!IMPORTANT]
> This package contains the main CRUD onchain data interactions

## Instructions Overview

- `CreateWorld`
- `UpdateWorld`
- `DeleteWorld`
- `SpawnEntity`
- `UpdateEntity`
- `DespawnEntity`

## CreateWorld

> [!WARNING]
> Currently, account and data validations for CreateWorld haven't yet been implemented. Do not use in Mainnet production use until a official release occurs.

**`CreateWorld`** creates the World account onchain that is central to the Regions and Entities that facilitate your FOCGs/AWs onchain state.

## UpdateWorld

> [!WARNING]
> Currently, account and data validations for CreateWorld haven't yet been implemented. Do not use in Mainnet production use until a official release occurs.

**`UpdateWorld`** creates the World account onchain that is central to the Regions and Entities that facilitate your FOCGs/AWs onchain state.

> [!IMPORTANT]
> **`UpdateWorld`** only allows you to update your `regions` and `entities` in the world and maintains an immutable `name`, `description`, `world_authority`, and `bump`.

## DeleteWorld

> [!WARNING]
> Currently, account and data validations for CreateWorld haven't yet been implemented. Do not use in Mainnet production use until a official release occurs.

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
