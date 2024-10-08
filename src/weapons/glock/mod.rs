use audio::{play_glock_audio, setup_glock_audio};
use bevy::prelude::*;

use animations::{load_glock_animation, setup_glock_animations};
use spawn::{despawn_glock, print_glock_position_system, spawn_glock, update_gun_rotation};

use crate::common::link_animations::link_multiple_animations;

use super::states::CurrentWeapon;

pub mod spawn;
pub mod animations;
pub mod audio;

pub struct GlockPlugin;

impl Plugin for GlockPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, (
            setup_glock_animations,
            setup_glock_audio
        ))
         .add_systems(Update, (
            spawn_glock,
            despawn_glock,
            link_multiple_animations,
            load_glock_animation,
            
         ))
         .add_systems(Update, (
            play_glock_audio
         ).run_if(in_state(CurrentWeapon::Glock)))
         .add_systems(Update, update_gun_rotation.after(spawn_glock));
    }
}