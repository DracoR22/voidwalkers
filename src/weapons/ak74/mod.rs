use animation::{load_ak74_animation, setup_ak74_animations};
use bevy::prelude::*;
use spawn::{spawn_ak74, update_gun_rotation};

use crate::game::link_animations::link_animations;

pub mod animation;
pub mod audio;
pub mod spawn;

pub struct AK74Plugin;

impl Plugin for AK74Plugin {
    fn build(&self, app: &mut App) {
        app
          .add_systems(Startup, setup_ak74_animations)
          .add_systems(Update, (
            spawn_ak74,
            load_ak74_animation,
            link_animations
          ))
          .add_systems(Update, update_gun_rotation.after(spawn_ak74));
    }
}