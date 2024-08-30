//! Parser Adapter
//!
//! Used for parsing Blueprint from the following
//! supported formats: `TOML`

use rush_core::blueprint::Blueprint;

/// Parser Trait
///
/// For flexibility to choose different blueprint
/// formats
pub trait Parser {
    fn parse_str() -> Blueprint {
        Blueprint::default()
    }

    fn parse_string() -> Blueprint {
        Blueprint::default()
    }
}
