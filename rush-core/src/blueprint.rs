use borsh::{BorshDeserialize, BorshSerialize};
use std::{
    cmp::{Eq, PartialEq},
    collections::BTreeMap,
};

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
#[derive(Clone, BorshDeserialize, BorshSerialize, Debug)]
pub enum ComponentValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl PartialEq for ComponentValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ComponentValue::String(v_self) => {
                let mut is_equal = false;
                if let ComponentValue::String(v_other) = other {
                    is_equal = v_self == v_other;
                }
                is_equal
            }

            ComponentValue::Integer(v_self) => {
                let mut is_equal = false;
                if let ComponentValue::Integer(v_other) = other {
                    is_equal = v_self == v_other;
                }
                is_equal
            }

            ComponentValue::Float(v_self) => {
                let mut is_equal = false;
                if let ComponentValue::Float(v_other) = other {
                    is_equal = v_self == v_other;
                }
                is_equal
            }

            ComponentValue::Boolean(v_self) => {
                let mut is_equal = false;
                if let ComponentValue::Boolean(v_other) = other {
                    is_equal = v_self == v_other;
                }
                is_equal
            }
        }
    }
}

impl Eq for ComponentValue {}

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
