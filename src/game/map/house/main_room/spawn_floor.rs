use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{common::entities::EntityType, game::map::house::components::FloorComponent};

pub fn spawn_floor(mut commands: Commands, asset_server: Res<AssetServer>) {
    let floor_model = asset_server.load("models/wooden_floor.glb#Scene0");

    let num_floors_x = 3; // Number of floors along the X axis
    let num_floors_z = 3; // Number of floors along the Z axis
    let floor_spacing = 600.0; // Distance between the center of each floor

    for x in 0..num_floors_x {
        for z in 0..num_floors_z {
            let x_position = (x as f32) * floor_spacing;
            let z_position = (z as f32) * floor_spacing;

            commands.spawn(SceneBundle {
                scene: floor_model.clone(),
                transform: Transform {
                    translation: Vec3::new(x_position, -1.0, z_position), // Position floors in a grid
                    scale: Vec3::splat(100.0),
                    rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2), // Rotate floor to lie flat
                    ..default()
                },
                ..default()
            })
            // .insert(floor_size)
            .insert(Collider::cuboid(10.0, 10.0, 0.05))
            .insert(FloorComponent)
            .insert(EntityType::Floor)
            .insert(RigidBody::Fixed);
        }
    }

}