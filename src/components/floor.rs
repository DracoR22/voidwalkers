use bevy::prelude::*;

#[derive(Component)]
pub struct Floor {
    pub width: f32,
    pub height: f32
}

impl Floor {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}