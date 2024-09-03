use super::blueprint::{Blueprint, ComponentValue, Entity, Region};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table, *};

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
