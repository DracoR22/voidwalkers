use bevy::prelude::*;
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use voidhunt::{cubes::systems::spawn::spawn_cube_system, house::systems::HousePlugin, player::PlayerPlugin, states::GameStatePlugin, window::WindowSetupPlugin};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(WindowSetupPlugin)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(GameStatePlugin)
    .add_plugins(HousePlugin)
    .add_plugins(PlayerPlugin)
    .add_systems(Startup, spawn_cube_system)
    .run();
}
