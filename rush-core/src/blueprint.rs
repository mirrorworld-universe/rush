use crate::error::CoreError;
use anyhow::{bail, Result};
use borsh::{BorshDeserialize, BorshSerialize};
use std::{
    cmp::{Eq, PartialEq},
    collections::BTreeMap,
    mem::discriminant,
};

// TODO: Consider using structs for Region and Entity
// with Display trait as oposed to Plain-Old-Data String
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

impl ComponentValue {
    /// Unwraps Enum to its parameter value
    ///
    /// Get String inside Enum parameter. Panics if
    /// the ComponentValue is not a String
    pub fn unwrap_string(self) -> String {
        use ComponentValue::*;

        match self {
            String(v) => v,
            _ => panic!("Not a String"),
        }
    }

    /// Unwraps Enum to its parameter value
    ///
    /// Get f64 inside Enum parameter. Panics if
    /// the ComponentValue is not an f64
    pub fn unwrap_float(self) -> f64 {
        use ComponentValue::*;

        match self {
            Float(v) => v,
            _ => panic!("Not a Float"),
        }
    }

    /// Unwraps Enum to its parameter value
    ///
    /// Get i64 inside Enum parameter. Panics if
    /// the ComponentValue is not an i64
    pub fn unwrap_int(self) -> i64 {
        use ComponentValue::*;

        match self {
            Integer(v) => v,
            _ => panic!("Not an Integer"),
        }
    }

    /// Unwraps Enum to its parameter value
    ///
    /// Get bool inside Enum parameter. Panics if
    /// the ComponentValue is not an bool
    pub fn unwrap_bool(self) -> bool {
        use ComponentValue::*;

        match self {
            Boolean(v) => v,
            _ => panic!("Not a Boolean"),
        }
    }
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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
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

    /// Preloads Region and Entity Keys
    ///
    /// The add_instance associated function will fail when preload
    /// is not called to preload the Region and Entity keys
    /// needed in the Instances table
    pub fn preload(&mut self, regions: Vec<Region>, entities: Vec<Entity>) {
        for region_name in regions.into_iter() {
            // preload Regions
            self.instances.insert(region_name.clone(), BTreeMap::new());

            // preload Entities
            let region_mut = self.instances.get_mut(&region_name).unwrap();
            for entity_name in entities.clone().into_iter() {
                region_mut.insert(entity_name, Vec::new());
            }
        }
    }

    pub fn add_entity(&mut self, name: Entity, component_types: ComponentTypeTree) {
        self.entities.insert(name, component_types);
    }

    pub fn add_region(&mut self, name: Region, entities: Vec<Entity>) {
        self.regions.insert(name, entities);
    }

    pub fn add_instance(
        &mut self,
        region: Region,
        entity: Entity,
        component_tree: ComponentTree,
    ) -> Result<()> {
        // get mutable region
        let region_mut = match self.instances.get_mut(&region) {
            // instance exists
            Some(e) => e,
            // insert and get, if not exists
            None => bail!(CoreError::RegionNotFound),
        };

        // get mutable entity
        let entity_mut = match region_mut.get_mut(&entity) {
            // instance exists
            Some(e) => e,
            // insert and get, if not exists
            None => bail!(CoreError::EntityNotFound),
        };

        // add entity instance to blueprint under region
        entity_mut.push(component_tree);

        Ok(())
    }

    // TODO: Add Validations
    /// Add Instance with default values
    ///
    /// Fetches the component tree structure from self.entities
    /// and creates an Instance with default values according
    /// to the Entity's ComponentTypeTree
    ///
    pub fn add_default_instance(&mut self, region: Region, entity: Entity) -> Result<u64> {
        // get mutable region
        let region_mut = match self.instances.get_mut(&region) {
            // instance exists
            Some(e) => e,
            // insert and get, if not exists
            None => bail!(CoreError::RegionNotFound),
        };

        // get mutable entity
        let entity_mut = match region_mut.get_mut(&entity) {
            // instance exists
            Some(e) => e,
            // insert and get, if not exists
            None => bail!(CoreError::EntityNotFound),
        };

        // get entity component type tree
        let component_type_tree = self.entities.get(&entity).unwrap();

        // populate component tree based on type tree
        let mut component_tree: ComponentTree = BTreeMap::new();
        for k in component_type_tree.keys() {
            let v_type = component_type_tree.get(k).unwrap().clone();

            let v: ComponentValue;
            if v_type == "String" {
                v = ComponentValue::String(String::default())
            } else if v_type == "f64" {
                v = ComponentValue::Float(f64::default())
            } else if v_type == "i64" {
                v = ComponentValue::Integer(i64::default())
            } else if v_type == "bool" {
                v = ComponentValue::Boolean(bool::default())
            } else {
                bail!(CoreError::UnsupportedDataType);
            }

            // push type default into component tree
            component_tree.insert(k.to_string(), v);
        }

        // add instance
        entity_mut.push(component_tree);

        // return index (nonce) of instance
        Ok(entity_mut.len() as u64)
    }

    pub fn get_instance(
        &mut self,
        region: Region,
        entity: Entity,
        nonce: u64,
    ) -> Result<ComponentTree> {
        // get mutable region
        let region_mut = match self.instances.get_mut(&region) {
            // instance exists
            Some(e) => e,
            // insert and get, if not exists
            None => bail!(CoreError::RegionNotFound),
        };

        // get mutable entity
        let entity_mut = match region_mut.get_mut(&entity) {
            // instance exists
            Some(e) => e,
            // insert and get, if not exists
            None => bail!(CoreError::EntityNotFound),
        };

        // get instance component tree
        let instance = entity_mut[nonce as usize].clone();

        Ok(instance)
    }

    pub fn get_component_value(
        &mut self,
        region: Region,
        entity: Entity,
        nonce: u64,
        component: Component,
    ) -> Result<ComponentValue> {
        let component_tree = match self.get_instance(region, entity, nonce) {
            Ok(ct) => ct,
            Err(e) => bail!("{}: {}", CoreError::InstanceNotFound, e),
        };

        let option_value = component_tree.get(&component);

        match option_value {
            Some(v) => Ok(v.clone()),
            None => bail!(CoreError::ComponentNotFound),
        }
    }

    pub fn set_component_value(
        &mut self,
        region: Region,
        entity: Entity,
        nonce: u64,
        component: Component,
        value: ComponentValue,
    ) -> Result<()> {
        // get mutable region
        let region_mut = match self.instances.get_mut(&region) {
            // instance exists
            Some(e) => e,
            // insert and get, if not exists
            None => bail!(CoreError::RegionNotFound),
        };

        // get mutable entity
        let entity_mut = match region_mut.get_mut(&entity) {
            // instance exists
            Some(e) => e,
            // insert and get, if not exists
            None => bail!(CoreError::EntityNotFound),
        };

        let component_tree = &mut entity_mut[nonce as usize];

        let option_value = component_tree.get_mut(&component);

        if option_value.is_none() {
            bail!(CoreError::ComponentNotFound);
        }

        let component_value = option_value.unwrap();

        // ensure they're the same ComponentValue variant
        if discriminant(&value) != discriminant(component_value) {
            bail!(CoreError::MismatchedDataType)
        }

        *component_value = value;

        Ok(())
    }
}
