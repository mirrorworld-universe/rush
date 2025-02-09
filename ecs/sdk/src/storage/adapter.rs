//! Storage Adapter
//!
//! Used for conveniently switching between storage options
//! for Rush SDKs

use anyhow::Result;
use rush_ecs_core::blueprint::{Component, ComponentValue, Entity, Region};

/// Storage Trait
///
/// Used as an adapter for different storage
/// provider. Enables the flexibility to choose a
/// different storage option when scaling
///
// @dev
// Storage is Send + Sync to enable concurrent parsing
// Storage is 'static for dynamic dispatch with Box
pub trait Storage: Send + Sync + 'static {
    /// Migrate data store from local definition to storage
    ///
    /// Used for initializing data storage
    ///
    /// (e.g. Uploading World into Solana)
    fn migrate(&mut self) -> Result<()>;
    /// Create new instance of Entity under a specific Region
    ///
    /// Returns u64 index of new instance in Blueprint instances
    /// mainly used for nonce
    fn create(&mut self, region: Region, entity: Entity) -> Result<u64>;
    /// Delete specific instance of Entity under a specific Region
    fn delete(&mut self, region: Region, entity: Entity, nonce: u64) -> Result<()>;

    /// Get value of a specific Component for a specific Instance
    fn get(
        &mut self,
        region: Region,
        entity: Entity,
        nonce: u64,
        component: Component,
    ) -> Result<ComponentValue>;

    /// Set value of a specific Component for a specific Instance
    fn set(
        &mut self,
        region: Region,
        entity: Entity,
        nonce: u64,
        component: Component,
        value: ComponentValue,
    ) -> Result<()>;
}
