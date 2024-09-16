use crate::{
    components::{AnimationIndices, AnimationTimer, Direction, DirectionTimer, Rect, State},
    walker::Walker,
    Config, Storage,
};
use bevy::prelude::*;
use rand::random;
use rush_core::blueprint::*;

#[derive(Clone, Component, Debug)]
pub struct Animal {
    pub name: String,
    pub state: State,
    pub direction: Direction,
    pub is_new_direction: bool,
    pub asset_path: String,
    pub rect: Rect,
    pub speed: f64,
    pub walker: Walker,
}

pub fn setup_animals(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    storage: ResMut<Storage>,
) {
    let animal_types = storage
        .storage
        .blueprint
        .instances
        .get("farm")
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    for animal_type in animal_types.iter() {
        if animal_type == "player" {
            continue;
        }

        let animals = storage
            .storage
            .blueprint
            .instances
            .get("farm")
            .unwrap()
            .get(animal_type)
            .unwrap()
            .to_vec();

        for animal in animals.iter() {
            // spawn player
            let x = animal.get("x").unwrap().clone().unwrap_float();
            let y = animal.get("y").unwrap().clone().unwrap_float();
            let w = animal.get("w").unwrap().clone().unwrap_float();
            let h = animal.get("h").unwrap().clone().unwrap_float();
            let speed = animal.get("speed").unwrap().clone().unwrap_float();
            let asset_path = animal.get("path").unwrap().clone().unwrap_string();

            let mut walker = Walker::new();
            walker.set(
                State::Standing,
                Direction::Up,
                AnimationIndices { first: 0, last: 1 },
            );
            walker.set(
                State::Standing,
                Direction::Down,
                AnimationIndices { first: 0, last: 1 },
            );
            walker.set(
                State::Standing,
                Direction::Left,
                AnimationIndices { first: 0, last: 1 },
            );
            walker.set(
                State::Standing,
                Direction::Right,
                AnimationIndices { first: 0, last: 1 },
            );
            walker.set(
                State::Walking,
                Direction::Up,
                AnimationIndices { first: 9, last: 16 },
            );
            walker.set(
                State::Walking,
                Direction::Down,
                AnimationIndices { first: 9, last: 16 },
            );
            walker.set(
                State::Walking,
                Direction::Left,
                AnimationIndices { first: 9, last: 16 },
            );
            walker.set(
                State::Walking,
                Direction::Right,
                AnimationIndices { first: 9, last: 16 },
            );

            let texture = asset_server.load(asset_path.clone());
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 9, 9, None, None);
            let atlas = texture_atlas_layouts.add(layout);
            let rect = Rect::from(x, y, w, h);
            let animation_timer = AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating));
            let direction_timer = DirectionTimer(Timer::from_seconds(1.0, TimerMode::Repeating));
            let sprite_bundle = SpriteBundle {
                transform: Transform::from_scale(Vec3::splat(1.0)),
                texture,
                ..default()
            };
            let texture_atlas = TextureAtlas {
                layout: atlas,
                index: 0,
            };

            let animal = Animal {
                name: animal_type.clone(),
                state: State::Standing,
                direction: Direction::Down,
                is_new_direction: true,
                asset_path,
                rect,
                speed,
                walker,
            };

            commands.spawn((
                animal,
                sprite_bundle,
                texture_atlas,
                animation_timer,
                direction_timer,
            ));
        }
    }
}

// Get events here and update (Non-Bevy Types, e.g. Player)
pub fn input_animals(
    time: Res<Time>,
    config: Res<Config>,
    mut query: Query<(&mut Animal, &mut DirectionTimer)>,
) {
    for (mut animal, mut timer) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let old_state = animal.state.clone();
            let old_direction = animal.direction.clone();

            animal.state = random();
            animal.direction = random();

            if old_state != animal.state || old_direction != animal.direction {
                animal.is_new_direction = true;
            }
        }

        if animal.rect.right() > (config.screen_width / 3.) as f64 / 2.0 {
            animal.direction = Direction::Left;
            animal.is_new_direction = true;
        } else if animal.rect.left() < -((config.screen_width / 3.) as f64 / 2.0) {
            animal.direction = Direction::Right;
            animal.is_new_direction = true;
        }

        if animal.rect.top() > (config.screen_height / 3.) as f64 / 2.0 {
            animal.direction = Direction::Down;
            animal.is_new_direction = true;
        } else if animal.rect.bottom() < -((config.screen_height / 3.) as f64 / 2.0) {
            animal.direction = Direction::Up;
            animal.is_new_direction = true;
        }
    }
}

// Update Storage here
pub fn set_animals(time: Res<Time>, mut storage: ResMut<Storage>, mut query: Query<&mut Animal>) {
    let animal_types = storage
        .storage
        .blueprint
        .instances
        .get("farm")
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    for animal_type in animal_types.iter() {
        if animal_type == "player" {
            continue;
        }

        let mut filtered = query
            .iter_mut()
            .filter(|x| x.name == *animal_type)
            .collect::<Vec<_>>();

        for (nonce, animal) in filtered.iter_mut().enumerate() {
            if animal.state == State::Walking {
                match animal.direction {
                    Direction::Up => {
                        let new_y = animal.rect.y + animal.speed * time.delta_seconds_f64();

                        storage
                            .storage
                            .set(
                                String::from("farm"),
                                animal_type.to_string(),
                                nonce as u64,
                                String::from("y"),
                                ComponentValue::Float(new_y),
                            )
                            .unwrap();

                        animal.rect.y = new_y;
                    }

                    Direction::Down => {
                        let new_y = animal.rect.y - animal.speed * time.delta_seconds_f64();

                        storage
                            .storage
                            .set(
                                String::from("farm"),
                                animal_type.to_string(),
                                nonce as u64,
                                String::from("y"),
                                ComponentValue::Float(new_y),
                            )
                            .unwrap();

                        animal.rect.y = new_y;
                    }
                    Direction::Left => {
                        let new_x = animal.rect.x - animal.speed * time.delta_seconds_f64();

                        storage
                            .storage
                            .set(
                                String::from("farm"),
                                animal_type.to_string(),
                                nonce as u64,
                                String::from("x"),
                                ComponentValue::Float(new_x),
                            )
                            .unwrap();

                        animal.rect.x = new_x;
                    }

                    Direction::Right => {
                        let new_x = animal.rect.x + animal.speed * time.delta_seconds_f64();

                        storage
                            .storage
                            .set(
                                String::from("farm"),
                                animal_type.to_string(),
                                nonce as u64,
                                String::from("x"),
                                ComponentValue::Float(new_x),
                            )
                            .unwrap();

                        animal.rect.x = new_x;
                    }
                }
            }
        }
    }
}

// Sync Bevy Entities by getting from Storage to update game
pub fn get_animals(mut storage: ResMut<Storage>, mut query: Query<(&Animal, &mut Transform)>) {
    let animal_types = storage
        .storage
        .blueprint
        .instances
        .get("farm")
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    for animal_type in animal_types.iter() {
        if animal_type == "player" {
            continue;
        }

        let filtered = query.iter_mut().filter(|x| x.0.name == *animal_type);

        for (nonce, (animal, mut animal_transform)) in filtered.enumerate() {
            let x = storage
                .storage
                .get(
                    String::from("farm"),
                    animal_type.to_string(),
                    nonce as u64,
                    String::from("x"),
                )
                .unwrap()
                .unwrap_float();
            let y = storage
                .storage
                .get(
                    String::from("farm"),
                    animal_type.to_string(),
                    nonce as u64,
                    String::from("y"),
                )
                .unwrap()
                .unwrap_float();

            animal_transform.translation.x = x as f32;
            animal_transform.translation.y = y as f32;

            // Flip animal based on LEFT and RIGHT
            if animal.direction == Direction::Right {
                animal_transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
            } else if animal.direction == Direction::Left {
                animal_transform.rotation = Quat::default();
            }
        }
    }
}

// Animate sprite based on current state
pub fn animate_animals(
    time: Res<Time>,
    mut query: Query<(&mut Animal, &mut TextureAtlas, &mut AnimationTimer)>,
) {
    for (mut animal, mut atlas, mut timer) in query.iter_mut() {
        let state = animal.state.clone();
        let direction = animal.direction.clone();

        let indices = animal.walker.get(state, direction);

        timer.tick(time.delta());
        if timer.just_finished() {
            if animal.is_new_direction {
                atlas.index = indices.first;
                animal.is_new_direction = false;
            } else {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}
