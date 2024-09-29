use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy_kira_audio::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use voidhunt::{cubes::systems::spawn::spawn_cube_system, enemies::EnemiesPlugin, game::blood::{cleanup_blood_effects, spawn_blood_mesh}, house::HousePlugin, player::PlayerPlugin, states::GameStatePlugin, weapons::WeaponsPlugin, window::WindowSetupPlugin};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(LogPlugin {
        filter: "error".into(),
        level: bevy::log::Level::ERROR,
        ..default()
    }))
    .add_plugins(HanabiPlugin)
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    .add_plugins(AudioPlugin)
    .add_plugins(WindowSetupPlugin)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(GameStatePlugin)
    .add_plugins(HousePlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(EnemiesPlugin)
    .add_plugins(WeaponsPlugin)

    .add_systems(Startup, spawn_cube_system)
    .add_systems(Startup, spawn_blood_mesh)
    .add_systems(Update, cleanup_blood_effects)
    .run();
}