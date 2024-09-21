use bevy::prelude::*;
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use voidhunt::{plugins::{floor_plugin::FloorPlugin, game_state_plugin::GameStatePlugin, player_plugin::PlayerPlugin, window_setup_plugin::WindowSetupPlugin}, systems::{camera_system::setup_cam, cube_system::spawn_cube_system}};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(WindowSetupPlugin)
    // .add_plugins(NoCameraPlayerPlugin)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(GameStatePlugin)
    .add_plugins(FloorPlugin)
    .add_plugins(PlayerPlugin)
    // .add_systems(Startup, setup_cam)
    .add_systems(Startup, spawn_cube_system)
    // .insert_resource(MovementSettings {
    //     sensitivity: 0.00015, // default: 0.00012
    //     speed: 1200.0, // default: 12.0
    // })
    .run();
}
