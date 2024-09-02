//! Parser Port for TOML File Format

use crate::{adapter::Parser, error::utils::ensure_syntax};
use anyhow::Result;
use rush_core::blueprint::{Blueprint, BlueprintString, Component, ComponentValue, Entity, Region};
use solana_sdk::hash::hash;
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

        // REGIONS

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

        // every region stated in the world table must have
        // a table of instances in the blueprint
        for region in regions.iter() {
            ensure_syntax(
                format!("Region {region} table must exist"),
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
        let entities = entity_table.keys().collect::<Vec<_>>();

        ensure_syntax(
            "Entity table must have at least 1 entity properties".to_string(),
            // not empty
            !entities.is_empty() &&
            // must be a table of properties e.g. { x = 0, y = 0 }
            entity_table[entities[0]].is_table(),
        );

        // // parse World's name
        // let world_name = table["world"]["name"]
        //     .as_str()
        //     .unwrap()
        //     .to_string();
        //
        // // create Blueprint let mut blueprint = Blueprint::new(world_name);
        //
        // // parse Entities
        // for entity in table["entity"].as_table()..keys() {}
        // let entities: Vec<Entity> = table["world"]["entities"]
        //     .as_array()
        //     .expect("World entities must be an array of String")
        //     .iter()
        //     .map(|e| {
        //         e.as_str()
        //             .expect("Entity name must be a valid String")
        //             .to_string()
        //     })
        //     .collect();
        //
        // // insert Entities and its Components into Blueprint
        // for entity in entities.into_iter() {
        //     let components: Vec<Component> = table[&entity]
        //         .as_table()
        //         .unwrap_or_else(|| panic!("Expected {entity} to be present in the Blueprint"))
        //         .keys()
        //         .cloned()
        //         .collect();
        //
        //     blueprint.add_entity(entity, components);
        // }
        //
        // // parse Regions
        // let regions: Vec<Region> = table["world"]["regions"]
        //     .as_array()
        //     .expect("World regions must be an array of String")
        //     .iter()
        //     .map(|e| {
        //         e.as_str()
        //             .expect("Region name must be a valid String")
        //             .to_string()
        //     })
        //     .collect();
        //
        // // insert Regions into Blueprint
        // for region in regions.into_iter() {
        //     let entities = table[&region]
        //     blueprint.add_region(region);
        // }
        //
        // // check which Region-Entity pair has instances
        // // Vec<(Region, Entity)>
        // let mut has_instances: Vec<(Region, Entity)> = Vec::new();
        // for region in blueprint.regions.iter() {
        //     for entity in blueprint.entities.keys() {
        //         // unwrap ok
        //         let list_of_instances = table["instances"][&region].get(entity);
        //         if list_of_instances.is_some() {
        //             has_instances.push((region.to_string(), entity.to_string()));
        //         }
        //     }
        // }
        //
        // // insert instances into Blueprint
        // for re_pair in has_instances.iter() {
        //     let (region, entity) = re_pair;
        //
        //     // unwrap ok, previously checked if there are instances for each pair
        //     let list_of_instances = table["instances"][&region]
        //         .get(entity)
        //         .unwrap()
        //         .as_array()
        //         .unwrap();
        //
        //     for instance in list_of_instances {
        //         // unwrap ok
        //         let mut components: Vec<(Component, ComponentValue)> = Vec::new();
        //         for component in blueprint.entities.get(entity).unwrap().iter() {
        //             let option_value = instance.get(component);
        //             let value = match option_value {
        //                 Some(v) => v,
        //                 // if None, get from defaults
        //                 None => table[entity].get(component).unwrap_or_else(|| {
        //                     panic!("component {component} from  {entity} must have a default value")
        //                 }),
        //             };
        //
        //             // parse supported data types
        //             let component_value = match value {
        //                 Value::String(v) => ComponentValue::String(v.clone()),
        //                 Value::Integer(v) => ComponentValue::Integer(*v),
        //                 Value::Boolean(v) => ComponentValue::Boolean(*v),
        //                 Value::Float(v) => ComponentValue::Float(*v),
        //                 _ => panic!("using an unsupported data type for instance"),
        //             };
        //
        //             components.push((component.to_string(), component_value));
        //         }
        //
        //         // insert Instance into Blueprint
        //         blueprint.add_instance(region.to_string(), entity.to_string(), components);
        //     }
        // }
        //
        // Ok(blueprint)
        Ok(Blueprint::new("tmp".to_string()))
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
