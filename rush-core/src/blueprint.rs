use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table, *};
use solana_sdk::hash::{hash, Hash};
use std::{collections::BTreeMap, fmt::Display};

pub type Region = String;
pub type Entity = String;
pub type Component = String;
pub type BlueprintString = String;
pub type Instance = BTreeMap<Component, ComponentValue>;

/// Blueprint of a World
///
/// Represents a World in programmable data
#[derive(Clone, Debug, Default)]
pub struct Blueprint {
    /// World's Name
    pub name: String,
    /// Entity name and Names of its Comoponents
    // TODO: Consider adding Default value in entities
    pub entities: BTreeMap<String, Vec<Component>>,
    /// Names of available Regions
    pub regions: Vec<String>,
    /// Container of all instances existing in the World
    ///
    /// Key: Hash(Region, Entity)
    ///
    pub instances: BTreeMap<Hash, Vec<Instance>>,
}

impl Blueprint {
    pub fn new(world_name: String) -> Self {
        Self {
            name: world_name,
            entities: BTreeMap::new(),
            regions: Vec::new(),
            instances: BTreeMap::new(),
        }
    }

    pub fn add_entity(&mut self, name: Entity, components: Vec<Component>) {
        self.entities.insert(name, components);
    }

    pub fn add_region(&mut self, name: Region) {
        self.regions.push(name);
    }

    pub fn add_instance(
        &mut self,
        region: Region,
        entity: Entity,
        components: Vec<(Component, ComponentValue)>,
    ) {
        let hash = hash(format!("{region}{entity}").as_bytes());

        let instances = match self.instances.get_mut(&hash) {
            // instance exists
            Some(e) => e,
            // insert and get, if not exists
            None => {
                self.instances.insert(hash, Vec::new());
                // unwrap ok
                self.instances.get_mut(&hash).unwrap()
            }
        };

        let mut instance: Instance = BTreeMap::new();
        for component_kv in components.into_iter() {
            let (name, value) = component_kv;
            let component = match instance.get_mut(&name) {
                Some(c) => c,
                None => {
                    instance.insert(name.clone(), value.clone());
                    // unwrap ok
                    instance.get_mut(&name).unwrap()
                }
            };

            *component = value;
        }

        // insert Instance
        instances.push(instance);
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
                Cell::new(self.regions.join(", ")),
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

        for entity in self.entities.keys() {
            // get component list
            let components_string = match self.entities.get(entity) {
                Some(components) => components.join(", "),
                None => String::from("(No registered components)"),
            };
            entity_table.add_row(vec![
                Cell::new(entity)
                    .fg(Color::Green)
                    .add_attribute(Attribute::Bold),
                Cell::new(components_string),
            ]);
        }

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

    #[test]
    fn test_blueprint_display() {
        let mut blueprint = Blueprint::new("Test World".to_string());

        // load mock regions
        blueprint.add_region("region1".to_string());
        blueprint.add_region("region2".to_string());

        // load mock entities
        blueprint.add_entity(
            "entity1".to_string(),
            vec!["x".to_string(), "y".to_string()],
        );
        blueprint.add_entity(
            "entity2".to_string(),
            vec!["width".to_string(), "height".to_string()],
        );

        // load mock instances
        let components = vec![
            ("x".to_string(), ComponentValue::Integer(0)),
            ("y".to_string(), ComponentValue::Integer(0)),
        ];
        blueprint.add_instance("region1".to_string(), "entity1".to_string(), components);

        println!("{blueprint}");
    }
}
