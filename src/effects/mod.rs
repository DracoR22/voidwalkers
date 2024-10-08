use bevy::prelude::*;
use blood_decal::{cleanup_blood_effects, spawn_blood_mesh};
use muzzle_flash::{spawn_muzzle_flash, update_muzzle_flash};

pub mod muzzle_flash;
pub mod components;
pub mod blood_decal;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup,  spawn_blood_mesh)
        .add_systems(Update, spawn_muzzle_flash)
        .add_systems(Update, (
            update_muzzle_flash,
            cleanup_blood_effects,
        ));
    }
}