//! Parser Port for TOML File Format

use crate::adapter::Parser;
use anyhow::Result;
use rush_core::blueprint::{Blueprint, BlueprintString, Component, Entity, Region};
use toml::Table;

/// TOML Blueprint Parser
///
/// This [`Parser`] expects a properly formatted
/// Blueprint [`String`] that follows:
///
/// Rush TOML DSL Specification
///
/// Example
///
/// ```toml
/// [world]
/// name = "Sonic's World"
/// entities = ["player"]
/// regions = ["farm", "house"]
///
/// [player] # (e.g. [<entity_name>])
/// name = "string"
/// x = 0
/// y = 0
/// w = 0
/// h = 0
/// speed = 0
///
/// [instances.farm]
/// player = [
///   # speed = 0, taken default value from [player] table
///   { name = "npc", x = 0, y = 0, w = 0, h = 0 }
/// ]
///
/// [instances.house]
/// player = [
///   { name = "npc", x = 0, y = 0, w = 0, h = 0, speed = 50 }
/// ]
/// ```
///
#[derive(Clone, Debug, Default)]
pub struct TomlParser {}

impl Parser for TomlParser {
    fn parse_string(&self, blueprint_string: BlueprintString) -> Result<Blueprint> {
        // expecting a valid TOML
        let table: Table = blueprint_string.parse::<Table>().expect("invalid TOML");

        // parse World's name
        let world_name = table["world"]["name"]
            .as_str()
            .expect("World name is required")
            .to_string();

        // create Blueprint
        let mut blueprint = Blueprint::new(world_name);

        // parse Entities
        let entities: Vec<Entity> = table["world"]["entities"]
            .as_array()
            .expect("World entities must be an array of String")
            .iter()
            .map(|e| {
                e.as_str()
                    .expect("Entity name must be a valid String")
                    .to_string()
            })
            .collect();

        // insert Entities and its Components into Blueprint
        for entity in entities.into_iter() {
            let components: Vec<Component> = table[&entity]
                .as_table()
                .unwrap_or_else(|| panic!("Expected {entity} to be present in the Blueprint"))
                .keys()
                .cloned()
                .collect();

            blueprint.add_entity(entity, components);
        }

        // parse Regions
        let regions: Vec<Region> = table["world"]["regions"]
            .as_array()
            .expect("World regions must be an array of String")
            .iter()
            .map(|e| {
                e.as_str()
                    .expect("Region name must be a valid String")
                    .to_string()
            })
            .collect();

        // Insert Regions into Blueprint
        for region in regions.into_iter() {
            blueprint.add_region(region);
        }

        // parse Instances

        println!("{:?}", blueprint);

        Ok(blueprint)
    }
}

mod tests {
    use super::*;
    use crate::utils::file_to_string;
    use std::path::Path;

    #[test]
    fn test_toml_parser() {
        let path = Path::new("mock/blueprint.toml");
        let blueprint_string = file_to_string(path);

        let toml_parser = TomlParser::default();
        let blueprint = toml_parser.parse_string(blueprint_string);
        assert!(true)
    }
}
