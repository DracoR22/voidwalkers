use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::floor::Floor;

pub fn spawn_floor_system(mut commands: Commands, asset_server: Res<AssetServer>) {
 let floor_size = Floor::new(1.0, 1.0);

 commands.spawn(SceneBundle {
    scene: asset_server.load("models/bathroom_floor.glb#Scene0"),
    transform: Transform {
        translation: Vec3::new(0.0, -1.0, 0.0),
        ..default()
    },
    ..default()
 })
 .insert(floor_size)
 .insert(Collider::cuboid(800.0, 0.05, 800.0))
 .insert(RigidBody::Fixed);
}