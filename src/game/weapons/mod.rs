use bevy::prelude::*;
use common::{reload_weapon, update_weapon_timer, WeaponFireTimer};
use glock::GlockPlugin;
use resources::{AK74Timer, CasingAudioTimer, GlockTimer, WeaponAudios};
use ak74::{audio::{play_ak74_audio,  setup_ak74_audio}, AK74Plugin};
use weapon_audio::setup_weapon_audio;

pub mod components;
pub mod resources;

pub mod ak74;
pub mod glock;
pub mod state;
pub mod common;
pub mod weapon_audio;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(AK74Timer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert_resource(GlockTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .insert_resource(WeaponFireTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert_resource(CasingAudioTimer {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
            shot_fired: false,
        })
        .add_systems(Startup, setup_weapon_audio)
        .add_systems(Update, reload_weapon)
        .add_systems(Update, update_weapon_timer)
        .add_plugins((
            AK74Plugin,
            GlockPlugin
        ));
    }
}