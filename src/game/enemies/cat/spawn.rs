use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::common::entities::EntityType;

use super::{components::CatComponent, resources::CatAnimations};

pub fn spawn_cat(mut commands: Commands, asset_server: Res<AssetServer>) {
   commands.insert_resource(CatAnimations(vec!(
    asset_server.load("models/cat.glb#Animation0"),
   )));

   let width = Vec3::new(3.0, 3.0, 3.0);

   commands.spawn((SceneBundle {
    scene: asset_server.load("models/cat.glb#Scene0"),
    transform: Transform {
        translation: Vec3::new(1350.0, 0.9, -360.0),
        scale: Vec3::splat(width.x),
        rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
        ..default()
    },
      ..default()
    }, 
  )) 
  .insert(RigidBody::Dynamic)
  .insert(Collider::cuboid(3.0, 3.0, 3.0))
  .insert(EntityType::Cat)
  .insert(CatComponent)
  .insert(GravityScale(70.0));
} 