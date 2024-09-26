use bevy::prelude::*;

use super::{components::Cat, resources::CatAnimations};

pub fn spawn_cat(mut commands: Commands, asset_server: Res<AssetServer>) {
   commands.insert_resource(CatAnimations(vec!(
    asset_server.load("models/cat.glb#Animation0"),
   )));

   commands.spawn((SceneBundle {
    scene: asset_server.load("models/cat.glb#Scene0"),
    transform: Transform {
        translation: Vec3::new(1350.0, 1.0, -360.0),
        scale: Vec3::splat(3.0),
        rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
        ..default()
    },
      ..default()
    }, 
      Cat,
  ));
} 