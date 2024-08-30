//! Parser Port for TOML File Format
use crate::adapter::Parser;

/// TOML Blueprint Parser
#[derive(Clone, Debug, Default)]
pub struct TomlParser {}

impl Parser for TomlParser {}
