use animations::{load_ak74_animation, setup_ak74_animations};
use audio::{play_ak74_audio, setup_ak74_audio};
use bevy::prelude::*;
use spawn::{despawn_ak74, respawn_ak74, spawn_ak74, update_gun_rotation};

use crate::common::{link_animations::link_multiple_animations, states::CurrentWeapon};


pub mod animations;
pub mod audio;
pub mod spawn;
pub mod components;
pub mod resources;

pub struct AK74Plugin;

impl Plugin for AK74Plugin {
    fn build(&self, app: &mut App) {
        app
          .add_systems(Startup, (
            setup_ak74_animations,
            setup_ak74_audio
          ))
          .add_systems(Update, (
            spawn_ak74,
            despawn_ak74,
            link_multiple_animations,
            load_ak74_animation,
          ))
          .add_systems(Update, (
            play_ak74_audio,
          ).run_if(in_state(CurrentWeapon::AK74)))
          .add_systems(Update, update_gun_rotation.after(spawn_ak74));
    }
}