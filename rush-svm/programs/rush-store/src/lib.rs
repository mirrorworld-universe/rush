#![forbid(unsafe_code)]

pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod macros;
pub mod pda;
pub mod processor;
pub mod state;

// Ensure tests don't affect binary with a compile flag
// #[cfg(feature = "test-sbf")]
pub mod tests;

solana_program::declare_id!("Aq2EAZ8i8UgKGaGzpSPhfvGxf4hkziymA4WqXrJ4NYu4");
