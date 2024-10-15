use bevy::prelude::*;

use ammo::{update_gun_ammo, write_gun_ammo};

pub mod ammo;

pub struct GamePlayUIPlugin;

impl Plugin for GamePlayUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, write_gun_ammo)
        .add_systems(Update, update_gun_ammo);
    }
}