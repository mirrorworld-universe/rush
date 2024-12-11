//! Auth Adapter
//!
//! Used for conveniently switching between authenication options
//! for Rush SDKs

use anyhow::Result;
use solana_sdk::signer::keypair::Keypair;

/// Auth Trait
///
/// Used as an adapter for different auth mechanisms.
/// Enables the flexibility to choose a different auth
/// mechanism when scaling
///
// @dev
// Auth is Send + Sync to enable concurrent parsing
// Auth is 'static for dynamic dispatch with Box
pub trait Auth: Send + Sync + 'static {
    // fn register(&self) -> Result<()>;

    /// Fetch Keypair from provided path
    fn signin(&self, path: &str) -> Result<Keypair>;

    // fn signout(&self);
}
