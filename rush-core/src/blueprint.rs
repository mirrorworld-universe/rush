use crate::utils::get_instances_table_display;

use super::utils::{get_entity_table_display, get_region_table_display, get_world_table_display};
use comfy_table::Table;
use std::{collections::BTreeMap, fmt::Display};

pub type Region = String;
pub type Entity = String;
pub type Component = String;
pub type ComponentPair = (Component, ComponentValue);
pub type ComponentTree = BTreeMap<Component, ComponentValue>;
pub type ComponentType = String;
pub type ComponentTypeTree = BTreeMap<Component, ComponentType>;
pub type BlueprintString = String;

/// Enum defining the supported dataset in the World
/// and how it maps with Rust data types
#[derive(Clone, Debug)]
pub enum ComponentValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

/// Blueprint of a World
///
/// Represents a World in programmable data
#[derive(Clone, Debug, Default)]
pub struct Blueprint {
    /// World's Name
    pub name: String,
    /// Region names and Entities that exist in it
    pub regions: BTreeMap<Region, Vec<Entity>>,
    /// Entity name and Names of its Components
    pub entities: BTreeMap<Entity, ComponentTypeTree>,
    /// Instances of different Entities in different Regions
    pub instances: BTreeMap<Region, BTreeMap<Entity, Vec<ComponentTree>>>,
}

impl Blueprint {
    pub fn new(world_name: String) -> Self {
        Self {
            name: world_name,
            entities: BTreeMap::new(),
            regions: BTreeMap::new(),
            instances: BTreeMap::new(),
        }
    }

    pub fn add_entity(&mut self, name: Entity, component_types: ComponentTypeTree) {
        self.entities.insert(name, component_types);
    }

    pub fn add_region(&mut self, name: Region, entities: Vec<Entity>) {
        self.regions.insert(name, entities);
    }

    pub fn add_instance(&mut self, region: Region, entity: Entity, component_tree: ComponentTree) {
        // get mutable region
        let region_mut = match self.instances.get_mut(&region) {
            // instance exists
            Some(e) => e,
            // insert and get, if not exists
            None => {
                self.instances.insert(region.clone(), BTreeMap::new());
                // unwrap ok
                self.instances.get_mut(&region).unwrap()
            }
        };

        // get mutable entity
        let entity_mut = match region_mut.get_mut(&entity) {
            // instance exists
            Some(e) => e,
            // insert and get, if not exists
            None => {
                region_mut.insert(entity.clone(), Vec::new());
                // unwrap ok
                region_mut.get_mut(&entity).unwrap()
            }
        };

        // add entity instance to blueprint under region
        entity_mut.push(component_tree);
    }
}

impl Display for Blueprint {
    ///
    /// Displays the blueprint into human-readable format via a
    /// comfy CLI table
    ///
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // get World table
        let world_table = get_world_table_display(self);

        // get per Region table
        let mut region_tables: Vec<Table> = Vec::new();
        for region in self.regions.keys() {
            let table = get_region_table_display(region, self);
            region_tables.push(table);
        }

        // get per Entity table
        let mut entity_tables: Vec<Table> = Vec::new();
        for entity in self.entities.keys() {
            let table = get_entity_table_display(entity, self);
            entity_tables.push(table);
        }

        // get Instances table
        let mut instances_table: Vec<Table> = Vec::new();
        for region in self.regions.keys() {
            if let Some(entities_in_region) = self.instances.get(region) {
                for entity in entities_in_region.keys() {
                    let table = get_instances_table_display(region, entity, self);
                    instances_table.push(table);
                }
            }
        }

        // convert to String
        let region_tables_string = region_tables
            .iter()
            .map(|t| format!("{t}"))
            .collect::<Vec<_>>()
            .join("\n\n");

        // convert to String
        let entity_tables_string = entity_tables
            .iter()
            .map(|t| format!("{t}"))
            .collect::<Vec<_>>()
            .join("\n\n");

        // convert to String
        let instances_table_string = instances_table
            .iter()
            .map(|t| format!("{t}"))
            .collect::<Vec<_>>()
            .join("\n\n");

        write!(
            f,
            "{world_table}\n\n{region_tables_string}\n\n{entity_tables_string}\n\n{instances_table_string}"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Strip escape codes from string and match
    #[test]
    fn test_blueprint_display() {
        let mut blueprint = Blueprint::new("Test World".to_string());

        let region1 = String::from("region1");
        let region2 = String::from("region2");
        let entity1 = String::from("entity1");
        let entity2 = String::from("entity2");

        // load mock regions
        blueprint.add_region(region1.clone(), vec![entity1.clone()]);
        blueprint.add_region(region2.clone(), vec![entity2.clone()]);
        // load mock entity1
        let mut component_type_tree1: ComponentTypeTree = BTreeMap::new();
        component_type_tree1.insert("x".to_string(), "int".to_string());
        component_type_tree1.insert("y".to_string(), "int".to_string());
        blueprint.add_entity(entity1.clone(), component_type_tree1);
        // load mock entity2
        let mut component_type_tree2: ComponentTypeTree = BTreeMap::new();
        component_type_tree2.insert("w".to_string(), "float".to_string());
        component_type_tree2.insert("h".to_string(), "float".to_string());
        blueprint.add_entity(entity2.clone(), component_type_tree2);
        // load mock instances1
        let mut component_tree1: ComponentTree = BTreeMap::new();
        component_tree1.insert("x".to_string(), ComponentValue::Integer(143));
        component_tree1.insert("y".to_string(), ComponentValue::Integer(143));
        blueprint.add_instance(region1.clone(), entity1.clone(), component_tree1);
        // load mock instances2
        let mut component_tree2: ComponentTree = BTreeMap::new();
        component_tree2.insert("w".to_string(), ComponentValue::Float(143.0));
        component_tree2.insert("h".to_string(), ComponentValue::Float(143.0));
        blueprint.add_instance(region2, entity2, component_tree2);

        println!("{blueprint}");
    }
}
