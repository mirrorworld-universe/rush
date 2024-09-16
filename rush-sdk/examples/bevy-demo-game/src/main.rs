mod animal;
mod components;
mod dungeon;
mod memory;
mod player;
mod walker;

use animal::*;
use bevy::{prelude::*, window::WindowResolution};
use clap::{builder::EnumValueParser, Arg, Command, ValueEnum};
use components::{AnimationIndices, AnimationTimer, Direction, Rect, State};
use dungeon::*;
use memory::Memory;
use player::*;
use rush_core::blueprint::*;
use walker::Walker;

#[derive(Resource)]
pub struct Config {
    pub region: String,
    pub screen_width: f32,
    pub screen_height: f32,
}

#[derive(Resource)]
struct Storage {
    pub storage: Memory,
}

const SCREEN_WIDTH: f32 = 500.;
const SCREEN_HEIGHT: f32 = 500.;

#[derive(Clone, Debug, ValueEnum)]
pub enum Mode {
    Farm,
    Dungeon,
}

#[tokio::main]
async fn main() {
    let matches = Command::new("gink")
        .about("Gink's demo game")
        .arg_required_else_help(true)
        .arg(
            Arg::new("MODE")
                .required(true)
                .long("mode")
                .short('m')
                .value_parser(EnumValueParser::<Mode>::new()),
        )
        .get_matches();

    // required flag ensures there is value
    let mode = matches.get_one::<Mode>("MODE").unwrap();

    match mode {
        Mode::Dungeon => {
            let mut memory = Memory::new(
                "Demo World".to_string(),
                "The Demo World by Gink".to_string(),
            );

            memory.migrate("blueprints/dungeon/spawned").unwrap();
            println!("{}", memory.blueprint);

            // load manifest onchain
            App::new()
                .insert_resource(ClearColor(Color::srgb(0.24, 0.25, 0.36)))
                .insert_resource(Config {
                    region: String::from("dungeon"),
                    screen_width: SCREEN_WIDTH,
                    screen_height: SCREEN_HEIGHT,
                })
                .insert_resource(Storage { storage: memory })
                .add_plugins(
                    DefaultPlugins
                        .set(WindowPlugin {
                            primary_window: Some(Window {
                                resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT)
                                    .with_scale_factor_override(3.0),
                                ..default()
                            }),
                            ..default()
                        })
                        .set(ImagePlugin::default_nearest()),
                )
                .add_systems(Startup, setup_player)
                .add_systems(Startup, setup_dungeon)
                .add_systems(
                    Update,
                    (input_player, set_player, get_player, animate_player),
                )
                .run();
        }
        Mode::Farm => {
            let mut memory = Memory::new(
                "Demo World".to_string(),
                "The Demo World by Gink".to_string(),
            );

            memory.migrate("blueprints/farm/spawned").unwrap();
            println!("{}", memory.blueprint);

            // load manifest onchain
            App::new()
                .insert_resource(ClearColor(Color::srgb(0.16, 0.40, 0.17)))
                .insert_resource(Config {
                    region: String::from("farm"),
                    screen_width: SCREEN_WIDTH,
                    screen_height: SCREEN_HEIGHT,
                })
                .insert_resource(Storage { storage: memory })
                .add_plugins(
                    DefaultPlugins
                        .set(WindowPlugin {
                            primary_window: Some(Window {
                                resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT)
                                    .with_scale_factor_override(3.0),
                                ..default()
                            }),
                            ..default()
                        })
                        .set(ImagePlugin::default_nearest()),
                )
                .add_systems(Startup, setup_player)
                .add_systems(Startup, setup_animals)
                .add_systems(
                    Update,
                    (input_player, set_player, get_player, animate_player),
                )
                .add_systems(
                    Update,
                    (input_animals, set_animals, get_animals, animate_animals),
                )
                .run();
        }
    };
}
