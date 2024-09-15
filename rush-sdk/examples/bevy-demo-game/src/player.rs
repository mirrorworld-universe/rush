use crate::components::{AnimationTimer, Direction, Rect, State};
use crate::walker::Walker;
use bevy::prelude::*;

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
