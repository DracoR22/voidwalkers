use audio::{play_glock_audio, setup_glock_audio};
use bevy::prelude::*;

use animations::{load_glock_animation, setup_glock_animations};
use components::GlockComponent;
use spawn::{despawn_glock, spawn_glock, update_gun_rotation};

use crate::common::{link_animations::link_multiple_animations, states::CurrentWeapon};

use super::{ak74::components::AK74Component};

pub mod spawn;
pub mod animations;
pub mod audio;
pub mod components;
pub mod resources;

pub const MAX_GLOCK_AMMO: usize = 125;
pub const GLOCK_MAGAZINE_SIZE: usize = 15;

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
            play_glock_audio,
            update_gun_rotation
         ).run_if(in_state(CurrentWeapon::Glock)))
         .add_systems(Update, update_gun_rotation.after(spawn_glock));
    }
}

pub trait Weapon {
    fn current_ammo(&self) -> usize;
    fn decrease_ammo(&mut self);
}

impl Weapon for GlockComponent {
    fn current_ammo(&self) -> usize {
        self.current_ammo
    }

    fn decrease_ammo(&mut self) {
        if self.current_ammo > 0 {
            self.current_ammo -= 1;
        }
    }
}

impl Weapon for AK74Component {
    fn current_ammo(&self) -> usize {
        self.current_ammo
    }

    fn decrease_ammo(&mut self) {
        if self.current_ammo > 0 {
            self.current_ammo -= 1;
        }
    }
}