use bevy::prelude::*;
use spawn::{despawn_glock, spawn_glock, update_gun_rotation};

pub mod spawn;
pub mod animations;

pub struct GlockPlugin;

impl Plugin for GlockPlugin {
    fn build(&self, app: &mut App) {
        app
         .add_systems(Update, (
            spawn_glock,
            despawn_glock,
            update_gun_rotation
         ));
    }
}