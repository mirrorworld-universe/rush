use std::collections::BTreeMap;

pub type Region = String;
pub type Entity = String;
pub type Component = String;
pub type BlueprintString = String;

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
    pub instances: BTreeMap<Region, BTreeMap<Entity, BTreeMap<Component, ComponentValue>>>,
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
}

/// Enum defining the supported dataset in the World
/// and how it maps with Rust data types
#[derive(Clone, Debug)]
pub enum ComponentValue {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
}
