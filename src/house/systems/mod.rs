use bevy::prelude::*;
use floor::spawn_floor_system;

pub mod floor;

pub struct HousePlugin;

impl Plugin for HousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_floor_system);
    }
}