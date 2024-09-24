use bevy::prelude::*;
use resources::AK74Timer;
use systems::audio::{play_ak74_audio, play_ak74_reload, setup_ak74_audio};

pub mod components;
pub mod resources;
pub mod systems;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(AK74Timer(Timer::from_seconds(0.3, TimerMode::Repeating)))
        .add_systems(Startup, setup_ak74_audio)
        .add_systems(Update, play_ak74_audio)
        .add_systems(Update, play_ak74_reload);
    }
}