use crate::error::StorageError;
use crate::storage::Storage;
use anyhow::{bail, Result};
use async_trait::async_trait;
use rush_core::blueprint::{Blueprint, Component, ComponentValue, Entity, Region};
use rush_parser::{toml::TomlParser, Loader};
use std::path::Path;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Memory {
    pub migrated: bool,
    pub blueprint: Blueprint,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            migrated: false,
            blueprint: Blueprint::new(String::from("In Memory Storage")),
        }
    }
}

#[async_trait]
impl Storage for Memory {
    async fn migrate(&mut self, path: &str) -> Result<()> {
        // TODO: Support other parsers. Pinned to TOML for now
        let toml_parser = TomlParser {};
        let loader = Loader::new(toml_parser);
        let path = Path::new(path);
        self.blueprint = loader.load_blueprint(path)?;
        self.migrated = true;
        Ok(())
    }

    async fn create(&mut self, region: Region, entity: Entity) -> Result<u64> {
        // migration guard
        if !self.migrated {
            bail!(StorageError::NotYetMigrated);
        }
        println!("CREATING BEFORE");

        // create new instance with default values
        let nonce = self.blueprint.add_default_instance(region, entity)?;

        println!("CREATING AFTER");

        // return index (nonce) of new instance
        Ok(nonce)
    }

    // TODO: Implement Delete instance
    async fn delete(&mut self, region: Region, entity: Entity) -> Result<()> {
        // migration guard
        if !self.migrated {
            bail!(StorageError::NotYetMigrated);
        }

        panic!("Not yet implemented");

        Ok(())
    }

    async fn get(
        &mut self,
        region: Region,
        entity: Entity,
        nonce: u64,
        component: Component,
    ) -> Result<ComponentValue> {
        // migration guard
        if !self.migrated {
            bail!(StorageError::NotYetMigrated);
        }

        let value = self
            .blueprint
            .get_component_value(region, entity, nonce, component)?;

        Ok(value)
    }

    async fn set(
        &mut self,
        region: Region,
        entity: Entity,
        nonce: u64,
        component: Component,
        value: ComponentValue,
    ) -> Result<()> {
        // migration guard
        if !self.migrated {
            bail!(StorageError::NotYetMigrated);
        }

        self.blueprint
            .set_component_value(region, entity, nonce, component, value)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rush_core::blueprint::*;
    use std::collections::BTreeMap;

    // TODO: Assert value
    // Happy path
    #[tokio::test]
    async fn test_migrate() {
        let mut memory = Memory::new();
        memory
            .migrate("mock/fixtures/memory/blueprint")
            .await
            .unwrap();
        println!("{:?}", memory);
        assert!(memory.migrated);
    }

    // Happy path
    #[tokio::test]
    async fn test_create() {
        let region = String::from("region1");
        let entity = String::from("entity1");
        let sample_blueprint = get_sample_blueprint();
        let mut memory = Memory::new();
        memory.blueprint = sample_blueprint;
        let _ = memory.create(region.clone(), entity.clone()).await.unwrap();
        let instances = memory
            .blueprint
            .instances
            .get(&region)
            .unwrap()
            .get(&entity)
            .unwrap();

        assert_eq!(instances.len(), 3);
        assert_eq!(
            *instances[2].get("x").unwrap(),
            ComponentValue::Integer(i64::default())
        );
        assert_eq!(
            *instances[2].get("y").unwrap(),
            ComponentValue::Integer(i64::default())
        );
    }

    // Happy path
    #[tokio::test]
    async fn test_delete() {}

    // Happy path
    #[tokio::test]
    async fn test_get() {}

    // Happy path
    #[tokio::test]
    async fn test_set() {}

    fn get_sample_blueprint() -> Blueprint {
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
        blueprint
            .add_instance(region1.clone(), entity1.clone(), component_tree1)
            .unwrap();
        // load mock instances2
        let mut component_tree2: ComponentTree = BTreeMap::new();
        component_tree2.insert("w".to_string(), ComponentValue::Float(143.0));
        component_tree2.insert("h".to_string(), ComponentValue::Float(143.0));
        blueprint
            .add_instance(region2, entity2, component_tree2)
            .unwrap();

        blueprint
    }
}
