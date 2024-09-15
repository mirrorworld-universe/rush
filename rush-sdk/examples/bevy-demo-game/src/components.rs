use bevy::prelude::*;

#[derive(Clone, Component, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum State {
    Standing,
    Walking,
}

#[derive(Clone, Component, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Component, Debug, Default, Eq, PartialEq)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Clone, Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

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
