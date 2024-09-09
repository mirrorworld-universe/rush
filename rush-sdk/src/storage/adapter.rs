//! Storage Adapter
//!
//! Used for conveniently switching between storage options
//! for Rush SDKs

use anyhow::Result;
use rush_core::blueprint::{Entity, Region};

/// Storage Trait
///
/// Used as an adapter for different storage
/// provider. Enables the flexibility to choose a
/// different storage option when scaling
///
// @dev
// Storage is Send + Sync to enable concurrent parsing
// Storage is 'static for dynamic dispatch with Box
pub trait IStorage: Send + Sync + 'static {
    /// Migrate data store from local definition to storage
    fn migrate(&self);
    /// Create new instance of Entity under a specific Region
    fn create(&self);
    /// Delete instance of Entity under a specific Region
    fn delete(&self);
    /// Get value of a specific Component for a specific Instance
    fn get(&self);
    /// Set value of a specific Component for a specific Instance
    fn set(&self);
}
