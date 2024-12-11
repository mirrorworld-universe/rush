#![forbid(unsafe_code)]
// #![cfg(target_os = "solana")]

pub mod instruction;
pub mod processor;
pub mod store_cpi;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

#[cfg(feature = "test-sbf")]
pub mod tests;

solana_program::declare_id!("EyQjSPB5s1drBzE5LFT4uwKzQCjndEJGexTkieQcVSJ9");
