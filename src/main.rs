use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy_kira_audio::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use voidhunt::{cubes::systems::spawn::spawn_cube_system, enemies::EnemiesPlugin, game::{blood::{cleanup_blood_effects, spawn_blood_mesh}, GamePlugin}, house::HousePlugin, player::PlayerPlugin, ui::GameUIPugin, weapons::{states::CurrentWeapon, WeaponsPlugin}, window::WindowSetupPlugin};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(LogPlugin {
        filter: "error".into(),
        level: bevy::log::Level::ERROR,
        ..default()
    }))
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
    .add_plugins(HanabiPlugin)
    .add_plugins(AudioPlugin)
    .add_plugins(WindowSetupPlugin)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(GamePlugin)
    .add_plugins(HousePlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(EnemiesPlugin)
    .add_plugins(WeaponsPlugin)
    .add_plugins(GameUIPugin)
    .init_state::<CurrentWeapon>()
    .add_systems(Startup, spawn_cube_system)
    .run();
}

#[derive(Asset, AsBindGroup, TypePath, Clone)]
struct LiquidMaterial {
    #[uniform(0)]
    time: f32,
}
