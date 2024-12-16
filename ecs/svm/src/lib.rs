pub mod error;
pub mod instruction;
pub mod macros;
pub mod pda;
pub mod state;

// Ensure unsupported crates from solana_sdk don't get
// imported into program specific code
#[cfg(not(target_os = "solana"))]
pub mod client;
