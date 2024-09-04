//! Rush Core Utilities

use super::blueprint::{
    Blueprint, ComponentTree, ComponentTypeTree, ComponentValue, Entity, Region,
};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table, *};
use std::{collections::BTreeMap, fmt::Display};

// implement Display trait for Blueprint
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

/// Get the printable World table from [`Blueprint`]
///
/// Constructs the World table from the [`Blueprint`] and
/// gives a comfy CLI table. Mainly used for Blueprint's
/// Display trait
///
pub fn get_world_table_display(blueprint: &Blueprint) -> Table {
    // build World table
    let mut world_table = Table::new();

    // count instances
    let mut instances_count = 0;
    for region in blueprint.regions.keys() {
        for entity in blueprint.entities.keys() {
            // if no instance in this region, skip region
            if !blueprint.instances.contains_key(region) {
                break;
            }

            // unwrap ok, previously checked
            let regional = blueprint.instances.get(region).unwrap();
            // if no instance in this entity, continue
            if !regional.contains_key(entity) {
                continue;
            }

            // unwrap ok, previously checked
            let instances = regional.get(entity).unwrap();
            let count = instances.iter().filter(|_| true).count();
            // update total instances count
            instances_count += count;
        }
    }

    world_table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("World")
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
            Cell::new(&blueprint.name),
        ])
        .add_row(vec![
            Cell::new("Regions").add_attribute(Attribute::Bold),
            Cell::new(blueprint.regions.keys().filter(|_| true).count()),
        ])
        .add_row(vec![
            Cell::new("Entities").add_attribute(Attribute::Bold),
            Cell::new(blueprint.entities.keys().filter(|_| true).count()),
        ])
        .add_row(vec![
            Cell::new("Instances").add_attribute(Attribute::Bold),
            Cell::new(instances_count),
        ]);

    world_table
}

/// Get the printable Region table from [`Blueprint`]
///
/// Constructs the Region table from the [`Blueprint`] and
/// gives a comfy CLI table. Mainly used for Blueprint's
/// Display trait
///
pub fn get_region_table_display(region: &Region, blueprint: &Blueprint) -> Table {
    // build Region table
    let mut regional_table = Table::new();
    regional_table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Region")
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
            Cell::new(region),
        ]);

    if let Some(entities) = blueprint.regions.get(region) {
        regional_table.add_row(vec![
            Cell::new("Entities").add_attribute(Attribute::Bold),
            Cell::new(entities.join(", ")),
        ]);
    }

    regional_table
}

/// Get the printable Entity table from [`Blueprint`]
///
/// Constructs the Entity table from the [`Blueprint`] and
/// gives a comfy CLI table. Mainly used for Blueprint's
/// Display trait
///
pub fn get_entity_table_display(entity: &Entity, blueprint: &Blueprint) -> Table {
    // build Region table
    let mut entity_table = Table::new();
    entity_table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Entity")
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
            Cell::new(entity),
        ]);

    if let Some(component_tree) = blueprint.entities.get(entity) {
        for (k, v) in component_tree {
            entity_table.add_row(vec![
                Cell::new(k).add_attribute(Attribute::Bold),
                Cell::new(v),
            ]);
        }
    }

    entity_table
}

/// Get the printable Instances table from [`Blueprint`]
///
/// Constructs the Instances table from the [`Blueprint`] and
/// gives a comfy CLI table. Mainly used for Blueprint's
/// Display trait
///
pub fn get_instances_table_display(
    region: &Region,
    entity: &Entity,
    blueprint: &Blueprint,
) -> Table {
    // build Region table
    let mut instances_table = Table::new();
    instances_table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Instances")
                .fg(Color::Green)
                .add_attribute(Attribute::Bold),
            Cell::new(format!("{region}, {entity}")),
        ]);

    // get instances from Region-Entity pair and construct
    // instances table from it
    //
    // get table for existing Entities in certain Region
    if let Some(entities_in_region) = blueprint.instances.get(region) {
        // get instances in certain Entity, in Region
        if let Some(instances) = entities_in_region.get(entity) {
            for (idx, instance) in instances.iter().enumerate() {
                // get all string pairs for combining later
                let mut instance_string_pairs: Vec<String> = Vec::new();
                // c_value = ComponentValue
                for (k, c_value) in instance {
                    let value_string = match c_value {
                        ComponentValue::String(v) => v.to_string(), // already a String
                        ComponentValue::Integer(v) => format!("{v}"),
                        ComponentValue::Float(v) => format!("{v}"),
                        ComponentValue::Boolean(v) => format!("{v}"),
                    };
                    instance_string_pairs.push(format!("{k} = {value_string}"));
                }
                // add row to instances_table
                instances_table.add_row(vec![
                    Cell::new(idx).add_attribute(Attribute::Bold),
                    // combine instance string pairs into 1 long string
                    Cell::new(instance_string_pairs.join(", ")),
                ]);
            }
        }
    }

    instances_table
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
