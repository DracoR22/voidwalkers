use bevy::prelude::*;

use spawn_assets::{open_door, spawn_assets};
use spawn_ceiling::spawn_ceiling;
use spawn_floor::spawn_floor;
use spawn_walls::spawn_walls;

pub mod spawn_floor;
pub mod spawn_walls;
pub mod spawn_assets;
pub mod spawn_ceiling;

pub struct MainRoomPlugin;

impl Plugin for MainRoomPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, (
            spawn_floor,
            spawn_assets,
            spawn_walls,
            spawn_ceiling
        )
        .chain()
    )
    .add_systems(Update, open_door);
    }
}