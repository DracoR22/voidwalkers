use animations::{load_ak74_animation, setup_ak74_animations};
use bevy::prelude::*;
use spawn::{despawn_ak74, respawn_ak74, spawn_ak74, update_gun_rotation};

use crate::game::link_animations::link_animations;

use super::components::CurrentWeaponState;

pub mod animations;
pub mod audio;
pub mod spawn;

pub struct AK74Plugin;

impl Plugin for AK74Plugin {
    fn build(&self, app: &mut App) {
        app
          .add_systems(Startup, setup_ak74_animations)
          .add_systems(Update, (
            spawn_ak74,
            despawn_ak74,
            respawn_ak74,
            link_animations,
            load_ak74_animation,
          ))
          // .add_systems(Update, (

          // ).run_if(in_state(CurrentWeaponState::AK74)))
          .add_systems(Update, update_gun_rotation.after(spawn_ak74));
    }
}