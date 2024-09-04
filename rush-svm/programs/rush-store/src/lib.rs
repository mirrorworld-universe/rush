#![forbid(unsafe_code)]
// #![cfg(target_os = "solana")]

pub mod instruction;
pub mod processor;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

#[cfg(feature = "test-sbf")]
pub mod tests;

solana_program::declare_id!("Aq2EAZ8i8UgKGaGzpSPhfvGxf4hkziymA4WqXrJ4NYu4");
