use bevy::prelude::*;
use blood_decal::{cleanup_blood_effects, spawn_blood_mesh};
use muzzle_flash::{update_muzzle_flash, setup_muzzle_flash};

use crate::player::systems::spawn::spawn_player_system;

pub mod muzzle_flash;
pub mod components;
pub mod blood_decal;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_muzzle_flash.after(spawn_player_system))
        .add_systems(Startup,  spawn_blood_mesh)
        .add_systems(Update, (
            update_muzzle_flash,
            cleanup_blood_effects,
        ));
    }
}