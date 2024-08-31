use solana_sdk::hash::{hash, Hash};
use std::collections::BTreeMap;

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

/// Enum defining the supported dataset in the World
/// and how it maps with Rust data types
#[derive(Clone, Debug)]
pub enum ComponentValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}
