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

// Blueprint { name: "Test World", entities: {"test_entity1": ["test_property"], "test_entity2": ["test_property"]}, regions: ["test_region1", "test_region2"], instances: {49C
// v49GYtkd8NRwcBYwmxBj8LR36qAYt3BSx5vgo38ZD: [{"test_property": Integer(0)}], AQsHsjb3ckqc7RtjZzTSs9H2ZXfvfghHjsNXsZWpjdNB: [{"test_property": Integer(0)}], DDXxZ8h3fCKRELQV7
// spk1aFso1eZZ1yAunkhKYkuksZi: [{"test_property": Integer(0)}]} }

// TODO: Finish Display
impl Display for Blueprint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // World table
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![Cell::new("World"), Cell::new(&self.name)])
            .add_row(vec![
                Cell::new("Regions"),
                Cell::new(self.regions.join(", ")),
            ])
            .add_row(vec![
                Cell::new("Entities"),
                Cell::new(self.entities.keys().cloned().collect::<Vec<_>>().join(", ")),
            ]);

        write!(f, "{}", table)
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
        blueprint.regions.push("region1".to_string());
        blueprint.regions.push("region2".to_string());
        blueprint.entities.insert("entity1".to_string(), Vec::new());
        blueprint.entities.insert("entity2".to_string(), Vec::new());
        println!("{blueprint}");
    }
}
