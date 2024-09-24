use bevy::prelude::*;
use main_room::MainRoomPlugin;

pub mod components;
pub mod main_room;

pub struct HousePlugin;

impl Plugin for HousePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MainRoomPlugin);
    }
}