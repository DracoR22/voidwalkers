use bevy::prelude::*;

use crate::systems::floor_system::spawn_floor_system;

pub struct FloorPlugin;

impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_floor_system);
    }
}