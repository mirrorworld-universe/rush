use crate::components::{AnimationIndices, AnimationTimer, Direction, Rect, State};
use crate::walker::Walker;
use crate::{Config, Storage};
use bevy::prelude::*;
use rush_core::blueprint::*;

#[derive(Clone, Component, Debug)]
pub struct Player {
    pub state: State,
    pub direction: Direction,
    pub is_new_direction: bool,
    pub asset_path: String,
    pub rect: Rect,
    pub speed: f64,
    pub walker: Walker,
}

pub fn setup_player(
    mut commands: Commands,
    config: Res<Config>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut storage: ResMut<Storage>,
) {
    commands.spawn(Camera2dBundle::default());

    let region = config.region.clone();

    // spawn player
    let x = storage
        .storage
        .get(region.clone(), String::from("player"), 0, String::from("x"))
        .unwrap()
        .unwrap_float();
    let y = storage
        .storage
        .get(region.clone(), String::from("player"), 0, String::from("y"))
        .unwrap()
        .unwrap_float();
    let w = storage
        .storage
        .get(region.clone(), String::from("player"), 0, String::from("w"))
        .unwrap()
        .unwrap_float();
    let h = storage
        .storage
        .get(region.clone(), String::from("player"), 0, String::from("h"))
        .unwrap()
        .unwrap_float();
    let speed = storage
        .storage
        .get(
            region.clone(),
            String::from("player"),
            0,
            String::from("speed"),
        )
        .unwrap()
        .unwrap_float();
    let asset_path = storage
        .storage
        .get(
            region.clone(),
            String::from("player"),
            0,
            String::from("path"),
        )
        .unwrap()
        .unwrap_string();

    let mut player_walker = Walker::new();
    player_walker.set(
        State::Standing,
        Direction::Up,
        AnimationIndices {
            first: 16,
            last: 21,
        },
    );
    player_walker.set(
        State::Standing,
        Direction::Down,
        AnimationIndices { first: 0, last: 5 },
    );
    player_walker.set(
        State::Standing,
        Direction::Left,
        AnimationIndices { first: 8, last: 13 },
    );
    player_walker.set(
        State::Standing,
        Direction::Right,
        AnimationIndices { first: 8, last: 13 },
    );
    player_walker.set(
        State::Walking,
        Direction::Up,
        AnimationIndices {
            first: 40,
            last: 45,
        },
    );
    player_walker.set(
        State::Walking,
        Direction::Down,
        AnimationIndices {
            first: 24,
            last: 29,
        },
    );
    player_walker.set(
        State::Walking,
        Direction::Left,
        AnimationIndices {
            first: 32,
            last: 37,
        },
    );
    player_walker.set(
        State::Walking,
        Direction::Right,
        AnimationIndices {
            first: 32,
            last: 37,
        },
    );

    let texture = asset_server.load(asset_path.clone());
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 8, 25, None, None);
    let atlas = texture_atlas_layouts.add(layout);
    let rect = Rect::from(x, y, w, h);
    let animation_timer = AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating));
    let sprite_bundle = SpriteBundle {
        transform: Transform::from_scale(Vec3::splat(1.0)),
        texture,
        ..default()
    };
    let texture_atlas = TextureAtlas {
        layout: atlas,
        index: 0,
    };

    let player = Player {
        state: State::Standing,
        direction: Direction::Down,
        is_new_direction: true,
        asset_path,
        rect,
        speed,
        walker: player_walker,
    };

    commands.spawn((player, sprite_bundle, texture_atlas, animation_timer));
}

// Get events here and update (Non-Bevy Types, e.g. Player)
pub fn input_player(keys: Res<ButtonInput<KeyCode>>, mut players: Query<&mut Player>) {
    for mut player in players.iter_mut() {
        if keys.pressed(KeyCode::KeyW) {
            player.state = State::Walking;
            player.direction = Direction::Up;
            if keys.just_pressed(KeyCode::KeyW) {
                player.is_new_direction = true;
            }
        } else if keys.pressed(KeyCode::KeyS) {
            player.state = State::Walking;
            player.direction = Direction::Down;
            if keys.just_pressed(KeyCode::KeyS) {
                player.is_new_direction = true;
            }
        } else if keys.pressed(KeyCode::KeyA) {
            player.state = State::Walking;
            player.direction = Direction::Left;
            if keys.just_pressed(KeyCode::KeyA) {
                player.is_new_direction = true;
            }
        } else if keys.pressed(KeyCode::KeyD) {
            player.state = State::Walking;
            player.direction = Direction::Right;
            if keys.just_pressed(KeyCode::KeyD) {
                player.is_new_direction = true;
            }
        } else {
            player.state = State::Standing;
            player.is_new_direction = true;
        }
    }
}

// Upate Storage here
pub fn set_player(
    time: Res<Time>,
    config: Res<Config>,
    mut storage: ResMut<Storage>,
    mut query: Query<&mut Player>,
) {
    let region = config.region.clone();
    for (nonce, mut player) in query.iter_mut().enumerate() {
        if player.state == State::Walking {
            match player.direction {
                Direction::Up => {
                    let new_y = player.rect.y + player.speed * time.delta_seconds_f64();

                    storage
                        .storage
                        .set(
                            region.clone(),
                            String::from("player"),
                            nonce as u64,
                            String::from("y"),
                            ComponentValue::Float(new_y),
                        )
                        .unwrap();

                    player.rect.y = new_y;
                }

                Direction::Down => {
                    let new_y = player.rect.y - player.speed * time.delta_seconds_f64();

                    storage
                        .storage
                        .set(
                            region.clone(),
                            String::from("player"),
                            nonce as u64,
                            String::from("y"),
                            ComponentValue::Float(new_y),
                        )
                        .unwrap();

                    player.rect.y = new_y;
                }
                Direction::Left => {
                    let new_x = player.rect.x - player.speed * time.delta_seconds_f64();

                    storage
                        .storage
                        .set(
                            region.clone(),
                            String::from("player"),
                            nonce as u64,
                            String::from("x"),
                            ComponentValue::Float(new_x),
                        )
                        .unwrap();

                    player.rect.x = new_x;
                }

                Direction::Right => {
                    let new_x = player.rect.x + player.speed * time.delta_seconds_f64();

                    storage
                        .storage
                        .set(
                            region.clone(),
                            String::from("player"),
                            nonce as u64,
                            String::from("x"),
                            ComponentValue::Float(new_x),
                        )
                        .unwrap();

                    player.rect.x = new_x;
                }
            }
        }
    }
}

// Sync Bevy Entities by getting from Storage to update game
pub fn get_player(
    config: Res<Config>,
    mut storage: ResMut<Storage>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let region = config.region.clone();

    for (nonce, (player, mut player_transform)) in query.iter_mut().enumerate() {
        let x = storage
            .storage
            .get(
                region.clone(),
                String::from("player"),
                nonce as u64,
                String::from("x"),
            )
            .unwrap()
            .unwrap_float();
        let y = storage
            .storage
            .get(
                region.clone(),
                String::from("player"),
                nonce as u64,
                String::from("y"),
            )
            .unwrap()
            .unwrap_float();

        player_transform.translation.x = x as f32;
        player_transform.translation.y = y as f32;

        // Flip player based on LEFT and RIGHT
        if player.direction == Direction::Left {
            player_transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        } else if player.direction == Direction::Right {
            player_transform.rotation = Quat::default();
        }
    }
}

// Animate sprite based on current state
pub fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut Player, &mut TextureAtlas, &mut AnimationTimer)>,
) {
    for (mut player, mut atlas, mut timer) in query.iter_mut() {
        let state = player.state.clone();
        let direction = player.direction.clone();

        let indices = player.walker.get(state, direction);

        timer.tick(time.delta());
        if timer.just_finished() {
            if player.is_new_direction {
                atlas.index = indices.first;
                player.is_new_direction = false;
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
