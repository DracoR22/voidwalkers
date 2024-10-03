use bevy::prelude::*;
use glock::GlockPlugin;
use resources::AK74Timer;
use ak74::{audio::{play_ak74_audio, play_ak74_reload, setup_ak74_audio}, AK74Plugin};

pub mod components;
pub mod resources;

pub mod ak74;
pub mod glock;
pub mod states;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(AK74Timer(Timer::from_seconds(0.3, TimerMode::Repeating)))
        .add_plugins((
            AK74Plugin,
            GlockPlugin
        ))
        .add_systems(Startup, setup_ak74_audio)
        .add_systems(Update, play_ak74_audio)
        .add_systems(Update, play_ak74_reload);
    }
}