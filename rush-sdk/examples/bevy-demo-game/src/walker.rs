use super::AnimationIndices;
use crate::components::{Direction, State};
use bevy::prelude::*;
use std::collections::BTreeMap;

#[derive(Clone, Component, Debug, Eq, PartialEq)]
pub struct Walker {
    pub indices: BTreeMap<State, BTreeMap<Direction, AnimationIndices>>,
}

impl Walker {
    pub const STATES: [State; 2] = [State::Standing, State::Walking];
    pub const DIRECTIONS: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    pub fn new() -> Self {
        let mut indices = BTreeMap::new();

        // ensure all keys are preloaded
        for s in Self::STATES.into_iter() {
            indices.insert(s.clone(), BTreeMap::new());
            let indices_mut = indices.get_mut(&s).unwrap();

            for d in Self::DIRECTIONS.into_iter() {
                indices_mut.insert(d, AnimationIndices::default());
            }
        }

        Self { indices }
    }

    pub fn set(&mut self, state: State, direction: Direction, animation_indices: AnimationIndices) {
        let indices = self
            .indices
            .get_mut(&state)
            .unwrap()
            .get_mut(&direction)
            .unwrap();
        *indices = animation_indices;
    }

    pub fn get(&mut self, state: State, direction: Direction) -> AnimationIndices {
        self.indices
            .get(&state)
            .unwrap()
            .get(&direction)
            .unwrap()
            .clone()
    }
}
