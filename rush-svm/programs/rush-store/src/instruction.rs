use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

/// RushStore Instruction List
///
/// For World Authority:
/// -
///
/// For Region Authority:
/// -
///
/// For Entity Authority:
/// -
///
#[repr(C)]
#[derive(
    BorshDeserialize, BorshSerialize, Clone, Debug, Eq, PartialEq, ShankContext, ShankInstruction,
)]
#[rustfmt::skip]
pub enum RushStoreInstruction {
    #[account(
        0,
        signer,
        name = "world_authority",
        desc = "World authority who has access to"
    )]
    CreateWorld,

    #[account(
        0,
        writable,
        name = "payer",
        desc = "Account description"
    )]
    UpdateWorld,

    #[account(
        0,
        writable,
        name = "payer",
        desc = "Account description"
    )]
    DeleteWorld,

    #[account(
        0,
        writable,
        name = "payer",
        desc = "Account description"
    )]
    SpawnEntity,

    #[account(
        0,
        writable,
        name = "payer",
        desc = "Account description"
    )]
    UpdateEntity,

    #[account(
        0,
        writable,
        name = "payer",
        desc = "Account description"
    )]
    DespawnEntity,
}
