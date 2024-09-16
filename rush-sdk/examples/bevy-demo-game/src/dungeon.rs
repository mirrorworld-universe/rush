use crate::{
    components::{AnimationIndices, AnimationTimer, Direction, DirectionTimer, Rect, State},
    walker::Walker,
    Config, Storage,
};
use bevy::prelude::*;
use rand::random;
use rush_core::blueprint::*;

#[derive(Clone, Component, Debug)]
pub struct Item {
    pub asset_path: String,
    pub rect: Rect,
}

pub fn setup_dungeon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut storage: ResMut<Storage>,
) {
    let items = storage
        .storage
        .blueprint
        .instances
        .get("dungeon")
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    for item in items.iter() {
        let instances = storage
            .storage
            .blueprint
            .instances
            .get("dungeon")
            .unwrap()
            .get(item)
            .unwrap()
            .to_vec();

        for (nonce, _) in instances.iter().enumerate() {
            let x = storage
                .storage
                .get(
                    String::from("dungeon"),
                    item.to_string(),
                    nonce as u64,
                    String::from("x"),
                )
                .unwrap()
                .unwrap_float();
            let y = storage
                .storage
                .get(
                    String::from("dungeon"),
                    item.to_string(),
                    nonce as u64,
                    String::from("y"),
                )
                .unwrap()
                .unwrap_float();
            let w = storage
                .storage
                .get(
                    String::from("dungeon"),
                    item.to_string(),
                    nonce as u64,
                    String::from("w"),
                )
                .unwrap()
                .unwrap_float();
            let h = storage
                .storage
                .get(
                    String::from("dungeon"),
                    item.to_string(),
                    nonce as u64,
                    String::from("h"),
                )
                .unwrap()
                .unwrap_float();
            let asset_path = storage
                .storage
                .get(
                    String::from("dungeon"),
                    item.to_string(),
                    nonce as u64,
                    String::from("path"),
                )
                .unwrap()
                .unwrap_string();

            let texture = asset_server.load(asset_path.clone());
            let sprite_bundle = SpriteBundle {
                transform: Transform::from_scale(Vec3::splat(1.0)),
                texture,
                ..default()
            };

            let rect = Rect::from(x, y, w, h);
            let item = Item { asset_path, rect };

            commands.spawn((item, sprite_bundle));
        }
    }
}
