use bevy::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Clone, Component, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum State {
    Standing,
    Walking,
}

impl Distribution<State> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> State {
        match rng.gen_range(0..=1) {
            0 => State::Standing,
            _ => State::Walking,
        }
    }
}

#[derive(Clone, Component, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

#[derive(Clone, Component, Debug, Default, Eq, PartialEq)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Clone, Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Clone, Component, Deref, DerefMut)]
pub struct DirectionTimer(pub Timer);

#[derive(Clone, Component, Debug, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rect {
    pub fn from(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }

    pub fn left(&self) -> f64 {
        self.x - self.w / 2.0
    }

    pub fn right(&self) -> f64 {
        self.x + self.w / 2.0
    }

    pub fn top(&self) -> f64 {
        self.y + self.h / 2.0
    }

    pub fn bottom(&self) -> f64 {
        self.y - self.h / 2.0
    }
}
