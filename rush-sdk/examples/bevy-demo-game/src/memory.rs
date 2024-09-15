use anyhow::{bail, Result};
use rush_core::blueprint::{Blueprint, Component, ComponentValue, Entity, Region};
use rush_parser::{toml::TomlParser, Loader};
use std::path::Path;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Memory {
    pub migrated: bool,
    pub blueprint: Blueprint,
}

impl Memory {
    pub fn new(world_name: String, world_description: String) -> Self {
        Self {
            migrated: false,
            blueprint: Blueprint::new(world_name, world_description),
        }
    }

    pub fn migrate(&mut self, path: &str) -> Result<()> {
        // TODO: Support other parsers. Pinned to TOML for now
        let toml_parser = TomlParser {};
        let loader = Loader::new(toml_parser);
        let path = Path::new(path);
        self.blueprint = loader.load_blueprint(path)?;
        self.migrated = true;
        Ok(())
    }

    pub fn create(&mut self, region: Region, entity: Entity) -> Result<u64> {
        // create new instance with default values
        let nonce = self.blueprint.add_default_instance(region, entity)?;

        // return index (nonce) of new instance
        Ok(nonce)
    }

    // TODO: Implement Delete instance
    // NOTE: In general, we need to consider the fact that deleting an instance
    // in the Memory store breaks the Vector indexing
    pub fn delete(&mut self, region: Region, entity: Entity, nonce: u64) -> Result<()> {
        panic!("Not yet implemented");

        Ok(())
    }

    pub fn get(
        &mut self,
        region: Region,
        entity: Entity,
        nonce: u64,
        component: Component,
    ) -> Result<ComponentValue> {
        let value = self
            .blueprint
            .get_component_value(region, entity, nonce, component)?;

        Ok(value)
    }

    pub fn set(
        &mut self,
        region: Region,
        entity: Entity,
        nonce: u64,
        component: Component,
        value: ComponentValue,
    ) -> Result<()> {
        self.blueprint
            .set_component_value(region, entity, nonce, component, value)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use rush_core::blueprint::*;
    use std::collections::BTreeMap;

    // TODO: Assert value
    // Happy path
    #[tokio::test]
    async fn test_memory_migrate() {
        let mut memory = Memory::new(
            String::from("Sonic's World"),
            String::from("This is Sonic's world"),
        );
        let path_str = "mock/fixtures/memory/blueprint.toml";
        memory.migrate(path_str).unwrap();
        assert!(memory.migrated);
    }

    // Happy path
    #[tokio::test]
    async fn test_memory_create() {
        let region1 = String::from("region1");
        let region2 = String::from("region2");
        let entity1 = String::from("entity1");
        let entity2 = String::from("entity2");

        let sample_blueprint = get_sample_blueprint();
        let mut memory = Memory::new(
            String::from("Sonic's World"),
            String::from("This is Sonic's world"),
        );
        memory.migrated = true;
        memory.blueprint = sample_blueprint.clone();

        let instance_nonce = memory.create(region1.clone(), entity1.clone()).unwrap();

        let region1_entity1_instances = memory
            .blueprint
            .instances
            .get(&region1)
            .unwrap()
            .get(&entity1)
            .unwrap();

        let region2_entity2_instances = memory
            .blueprint
            .instances
            .get(&region2)
            .unwrap()
            .get(&entity2)
            .unwrap();

        assert_eq!(instance_nonce, 2); // 2nd instance in region1,entity1
        assert_eq!(region1_entity1_instances.len(), 2);
        assert_eq!(region2_entity2_instances.len(), 1);
        assert_eq!(
            *region1_entity1_instances[1].get("x").unwrap(),
            ComponentValue::Integer(i64::default())
        );
        assert_eq!(
            *region1_entity1_instances[1].get("y").unwrap(),
            ComponentValue::Integer(i64::default())
        );
    }

    // Happy path
    // #[tokio::test]
    // async fn test_delete() {}

    // Happy path
    #[tokio::test]
    async fn test_memory_get() {
        let region1 = String::from("region1");
        let entity1 = String::from("entity1");
        let component = String::from("x");
        let nonce = 0;

        let sample_blueprint = get_sample_blueprint();
        let mut memory = Memory::new(
            String::from("Sonic's World"),
            String::from("This is Sonic's world"),
        );
        memory.migrated = true;
        memory.blueprint = sample_blueprint.clone();

        let value = memory.get(region1, entity1, nonce, component).unwrap();

        let expected_parameter = 143;
        assert_matches!(value, ComponentValue::Integer(expected_parameter));
        assert_eq!(value.unwrap_int(), expected_parameter);
    }

    // Happy path
    #[tokio::test]
    async fn test_memory_set() {
        let region1 = String::from("region1");
        let entity1 = String::from("entity1");
        let component = String::from("x");
        let nonce = 0;

        let sample_blueprint = get_sample_blueprint();
        let mut memory = Memory::new(
            String::from("Sonic's World"),
            String::from("This is Sonic's World"),
        );
        memory.migrated = true;
        memory.blueprint = sample_blueprint.clone();

        let expected_parameter = 1337;
        let expected_value = ComponentValue::Integer(expected_parameter);
        memory
            .set(
                region1.clone(),
                entity1.clone(),
                nonce,
                component.clone(),
                expected_value,
            )
            .unwrap();

        let value = memory
            .blueprint
            .instances
            .get(&region1)
            .unwrap()
            .get(&entity1)
            .unwrap()[nonce as usize]
            .get(&component)
            .unwrap();

        assert_matches!(value, _expected_value);

        assert_eq!(value.clone().unwrap_int(), expected_parameter);
    }

    fn get_sample_blueprint() -> Blueprint {
        let mut blueprint = Blueprint::new(
            String::from("Test World"),
            String::from("This is Sonic's World"),
        );

        let region1 = String::from("region1");
        let region2 = String::from("region2");
        let entity1 = String::from("entity1");
        let entity2 = String::from("entity2");

        // preload Region and Entity keys
        blueprint.preload(
            vec![region1.clone(), region2.clone()],
            vec![entity1.clone(), entity2.clone()],
        );

        // load mock regions
        blueprint.add_region(region1.clone(), vec![entity1.clone()]);
        blueprint.add_region(region2.clone(), vec![entity2.clone()]);
        // load mock entity1
        let mut component_type_tree1: ComponentTypeTree = BTreeMap::new();
        component_type_tree1.insert("x".to_string(), "i64".to_string());
        component_type_tree1.insert("y".to_string(), "i64".to_string());
        blueprint.add_entity(entity1.clone(), component_type_tree1);
        // load mock entity2
        let mut component_type_tree2: ComponentTypeTree = BTreeMap::new();
        component_type_tree2.insert("w".to_string(), "f64".to_string());
        component_type_tree2.insert("h".to_string(), "f64".to_string());
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
