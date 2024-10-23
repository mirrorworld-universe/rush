//! Parser Port for TOML File Format

use crate::{adapter::Parser, error::utils::ensure_syntax};
use anyhow::Result;
use rush_core::blueprint::{
    Blueprint, BlueprintString, Component, ComponentType, ComponentValue, Entity,
};
use std::collections::BTreeMap;
use toml::{Table, Value};

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
/// description = "This is Sonic's world"
/// regions = ["farm", "house"]
///
/// [entity]
/// player = { name = "String", x = "f64", y = "f64", w = "f64", h = "f64", speed = "f64" }
/// apple = { x = "f64", y = "f64"}
///
/// [farm]
/// player = [
///    { name = "npc", x = 0.0, y = 0.0, w = 0.0, h = 0.0, speed = 0.0 }
/// ]
/// apple = [
///     { x = 0, y = 0}
/// ]
///
/// [house]
/// player = [
///     { name = "npc", x = 0.0, y = 0.0, w = 0.0, h = 0.0, speed = 0.0 }
/// ]
/// ```
///
#[derive(Clone, Debug, Default)]
pub struct TomlParser {}

impl Parser for TomlParser {
    fn parse_string(&self, blueprint_string: BlueprintString) -> Result<Blueprint> {
        // expecting a valid TOML
        let table: Table = blueprint_string.parse::<Table>().expect("invalid TOML");

        // ensure syntax for top-level Tables and Properties are met

        // WORLD

        ensure_syntax(
            "World table must exist".to_string(),
            table.contains_key("world"),
        );
        ensure_syntax(
            "World table must be a table".to_string(),
            table["world"].is_table(),
        );

        let world_table = table["world"].as_table().unwrap();

        ensure_syntax(
            "World must have a name".to_string(),
            world_table.contains_key("name"),
        );

        ensure_syntax(
            "World name must be a string".to_string(),
            world_table["name"].is_str(),
        );

        ensure_syntax(
            "World must have a description".to_string(),
            world_table.contains_key("description"),
        );

        ensure_syntax(
            "World description must be a string".to_string(),
            world_table["description"].is_str(),
        );

        ensure_syntax(
            "World must have a regions property".to_string(),
            world_table.contains_key("regions"),
        );
        ensure_syntax(
            "World regions property must be an array".to_string(),
            world_table["regions"].is_array(),
        );
        ensure_syntax(
            "World must have at least 1 region".to_string(),
            !world_table["regions"].as_array().unwrap().is_empty(),
        );
        ensure_syntax(
            "World regions property must be an array of strings".to_string(),
            world_table["regions"].as_array().unwrap()[0].is_str(),
        );

        // get regions into Vec<String>
        let regions = world_table["regions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r.as_str().unwrap().to_string()) // unwrap ok
            .collect::<Vec<_>>();

        // REGIONS

        // every region stated in the world table must have
        // a table of instances in the blueprint
        for region in regions.iter() {
            ensure_syntax(
                format!("Region {region} table must exist"),
                // certain region exists
                table.contains_key(region),
            );
        }

        // ENTITY

        ensure_syntax(
            "Enttiy table must exist".to_string(),
            table.contains_key("entity"),
        );
        ensure_syntax(
            "Entity table must be a table".to_string(),
            table["entity"].is_table(),
        );

        let entity_table = table["entity"].as_table().unwrap();
        let entities = entity_table.keys().cloned().collect::<Vec<_>>();

        ensure_syntax(
            "Entity table must have at least 1 entity properties".to_string(),
            // not empty
            !entities.is_empty() &&
            // must be a table of properties e.g. { x = 0, y = 0 }
            entity_table[&entities[0]].is_table(),
        );

        // parse World's name
        let world_name = world_table["name"].as_str().unwrap().to_string();
        let world_description = world_table["description"].as_str().unwrap().to_string();

        // create Blueprint
        let mut blueprint = Blueprint::new(world_name, world_description);

        // TODO: Move this closer to Load Instances
        // preload Instance Keys
        blueprint.preload(regions.clone(), entities.clone());

        // load Regions into World
        for region_name in regions.iter() {
            // load into World tree
            if let Some(region_table) = table[region_name].as_table() {
                // get entities from keys in the table
                let entities = region_table.keys().cloned().collect::<Vec<Entity>>();
                blueprint.add_region(region_name.clone(), entities);
            }
        }

        // load Entities into World
        for entity_name in entities.into_iter() {
            if let Some(component_table) = entity_table[&entity_name].as_table() {
                // load Entities
                let mut component_type_tree: BTreeMap<Component, ComponentType> = BTreeMap::new();
                for component_name in component_table.keys() {
                    // unwrap ok, a value is expected
                    let value = component_table
                        .get(component_name)
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string();
                    component_type_tree.insert(component_name.to_string(), value);
                }
                blueprint.add_entity(entity_name, component_type_tree);
            }
        }

        // TODO: Refactor nested For loops for readability
        //
        // load Instances
        //
        // In blueprint.rs:
        //  pub instances: BTreeMap<Region, BTreeMap<Entity, Vec<ComponentTree>>>,

        let blueprint_regions = blueprint.regions.clone();

        for region_name in regions.into_iter() {
            // if there are entities in region
            if let Some(entities_in_region) = blueprint_regions.get(&region_name) {
                // get each entity in region
                for entity_name in entities_in_region.iter() {
                    if let Some(instances) = table[&region_name][entity_name].as_array() {
                        for instance in instances.iter() {
                            // build each entity's component tree
                            if let Some(entity_components) = instance.as_table() {
                                let mut component_tree: BTreeMap<Component, ComponentValue> =
                                    BTreeMap::new();

                                // get (component, value) pairs
                                for (toml_component, toml_value) in entity_components.into_iter() {
                                    let component = toml_component.to_string();
                                    let value = match toml_value {
                                        Value::String(v) => ComponentValue::String(v.to_string()),
                                        Value::Float(v) => ComponentValue::Float(*v),
                                        Value::Integer(v) => ComponentValue::Integer(*v),
                                        Value::Boolean(v) => ComponentValue::Boolean(*v),
                                        _ => panic!("Unsupported data type"),
                                    };

                                    component_tree.insert(component, value);
                                }

                                blueprint.add_instance(
                                    region_name.clone(),
                                    entity_name.to_string(),
                                    component_tree,
                                )?;
                            }
                        }
                    }
                }
            }
        }

        Ok(blueprint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::file_to_string;
    use std::path::Path;

    #[test]
    fn test_toml_parser_file() {
        let path = Path::new("mock/fixtures/ports/blueprint.toml");
        let blueprint_string = file_to_string(path);

        let toml_parser = TomlParser::default();
        let blueprint = toml_parser.parse_string(blueprint_string).unwrap();
        println!("{:?}", blueprint);
        // TODO: Assert value
        assert!(true)
    }
}
