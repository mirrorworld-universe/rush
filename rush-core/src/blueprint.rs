use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table, *};
use std::{collections::BTreeMap, fmt::Display};

pub type Region = String;
pub type Entity = String;
pub type Component = String;
pub type ComponentPair = (Component, ComponentValue);
pub type ComponentTree = BTreeMap<Component, ComponentValue>;
pub type ComponentType = String;
pub type ComponentTypeTree = BTreeMap<Component, ComponentType>;
pub type BlueprintString = String;

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

// TODO: Finish Display, display Instances
impl Display for Blueprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // build World table
        let mut world_table = Table::new();
        world_table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("World")
                    .fg(Color::Green)
                    .add_attribute(Attribute::Bold),
                Cell::new(&self.name),
            ])
            .add_row(vec![
                Cell::new("Regions")
                    .fg(Color::Green)
                    .add_attribute(Attribute::Bold),
                Cell::new(self.regions.keys().cloned().collect::<Vec<_>>().join(", ")),
            ])
            .add_row(vec![
                Cell::new("Entities")
                    .fg(Color::Green)
                    .add_attribute(Attribute::Bold),
                Cell::new(self.entities.keys().cloned().collect::<Vec<_>>().join(", ")),
            ]);

        // list down Entities and their properties
        let mut entity_table = Table::new();
        entity_table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![Cell::new("Entity"), Cell::new("Components")]);

        // for entity in self.entities.keys() {
        //     // get component list
        //     let components_string = match self.entities.get(entity) {
        //         Some(components) => components.join(", "),
        //         None => String::from("(No registered components)"),
        //     };
        //     entity_table.add_row(vec![
        //         Cell::new(entity)
        //             .fg(Color::Green)
        //             .add_attribute(Attribute::Bold),
        //         Cell::new(components_string),
        //     ]);
        // }

        write!(f, "{world_table}\n\n{entity_table}")
    }
}

/// Enum defining the supported dataset in the World
/// and how it maps with Rust data types
#[derive(Clone, Debug)]
pub enum ComponentValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Blueprint Display test
    #[test]
    fn test_blueprint_display() {
        // let mut blueprint = Blueprint::new("Test World".to_string());
        //
        // // load mock regions
        // blueprint.add_region("region1".to_string());
        // blueprint.add_region("region2".to_string());
        //
        // // load mock entities
        // blueprint.add_entity(
        //     "entity1".to_string(),
        //     vec!["x".to_string(), "y".to_string()],
        // );
        // blueprint.add_entity(
        //     "entity2".to_string(),
        //     vec!["width".to_string(), "height".to_string()],
        // );
        //
        // // load mock instances
        // let components = vec![
        //     ("x".to_string(), ComponentValue::Integer(0)),
        //     ("y".to_string(), ComponentValue::Integer(0)),
        // ];
        // blueprint.add_instance("region1".to_string(), "entity1".to_string(), components);
        //
        // println!("{blueprint}");
    }
}
