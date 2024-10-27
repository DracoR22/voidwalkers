use bevy::prelude::*;
use bevy_kira_audio::prelude::AudioSource;

use common::{reload_weapon, spawn_weapons, update_weapon_timer, WeaponFireTimer};
use glock::{resources::GlockTimer, GlockPlugin};
use ak74::{audio::{play_ak74_audio,  setup_ak74_audio}, resources::AK74Timer, AK74Plugin};
use weapon_audio::setup_weapon_audio;
use weapon_manager::{equip_weapon, unequip_weapon};

use crate::common::entities::WeaponType;

pub mod ak74;
pub mod glock;
pub mod common;
pub mod weapon_audio;
pub mod weapon_manager;

#[derive(Component)]
pub struct HasWeapon(pub WeaponType);

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
        .add_systems(Startup, spawn_weapons)
        .add_systems(Startup, setup_weapon_audio)
        .add_systems(Update, (
            reload_weapon,
            equip_weapon,
            unequip_weapon
        ))
        .add_systems(Update, update_weapon_timer)
        .add_plugins((
            AK74Plugin,
            GlockPlugin
        ));
    }
}

#[derive(Resource)]
pub struct CasingAudioTimer {
    pub timer: Timer,
    pub shot_fired: bool,
}

#[derive(Resource, Default)]
pub struct WeaponAudios(pub Vec<Handle<AudioSource>>);