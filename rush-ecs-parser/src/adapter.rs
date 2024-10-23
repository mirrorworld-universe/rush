//! Parser Adapter
//!
//! Used for parsing Blueprint from the following
//! supported formats: `TOML`

use anyhow::Result;
use rush_ecs_core::blueprint::{Blueprint, BlueprintString};

/// Parser Trait
///
/// Used as an adapter for different Blueprint file
/// formats. Enables the flexibility to choose a
/// different DSL
///
// @dev
// Parser is Send + Sync to enable concurrent parsing
// Parser is 'static for dynamic dispatch with Box
pub trait Parser: Send + Sync + 'static {
    /// Parse [`String`] to [`Blueprint`]
    fn parse_string(&self, blueprint_string: BlueprintString) -> Result<Blueprint>;

    // TODO: Implement
    // Parse [`str`] to [`Blueprint`]
    // fn parse_str(path: &Path) -> Result<Blueprint>;
}
