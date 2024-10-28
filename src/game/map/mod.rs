use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use skybox::{daylight_cycle, setup_environment};

pub mod skybox;
pub mod audio;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(AtmospherePlugin)
        .insert_resource(AtmosphereModel::default())
        .insert_resource(CycleTimer(Timer::new(
            bevy::utils::Duration::from_millis(500),
             TimerMode::Repeating,
        )))

        .add_systems(Startup, setup_environment)
        .add_systems(Update, daylight_cycle);
    }
}

#[derive(Component)]
pub struct Sun;

#[derive(Resource)]
pub struct CycleTimer(pub Timer);
