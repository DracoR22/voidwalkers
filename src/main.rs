use std::env;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use voidhunt::{cubes::systems::spawn::spawn_cube_system, house::HousePlugin, player::PlayerPlugin, states::GameStatePlugin, weapons::WeaponsPlugin, window::WindowSetupPlugin};

fn main() {
    env::set_var("RUST_LOG", "info,bevy_kira_audio=error");
    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    .add_plugins(AudioPlugin)
    .add_plugins(WindowSetupPlugin)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(GameStatePlugin)
    .add_plugins(HousePlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(WeaponsPlugin)
    // .add_systems(Startup, start_background_audio)
    .add_systems(Startup, spawn_cube_system)
    .run();
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play(asset_server.load("audios/ak74.ogg")).looped();
}