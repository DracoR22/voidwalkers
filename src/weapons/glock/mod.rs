use animations::{load_glock_animation, setup_glock_animations};
use bevy::prelude::*;
use spawn::{despawn_glock, spawn_glock, update_gun_rotation};

use crate::game::link_animations::{link_animations, link_multiple_animations};

pub mod spawn;
pub mod animations;

pub struct GlockPlugin;

impl Plugin for GlockPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_glock_animations)
         .add_systems(Update, (
            spawn_glock,
            despawn_glock,
            link_multiple_animations,
            load_glock_animation
         ))
         .add_systems(Update, update_gun_rotation.after(spawn_glock));
    }
}