use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::house::components::Floor;

pub fn spawn_floor_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let floor_model = asset_server.load("models/wooden_floor.glb#Scene0");
    let wall_model: Handle<Scene> = asset_server.load("models/wall1.glb#Scene0");

    // Define how many floors to spawn and the gap between them
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
            .insert(Collider::cuboid(800.0, 800.0, 0.05))
            .insert(RigidBody::Fixed);
        }
    }

    let initial_position = Vec3::new(1577.0, 0.0, -570.0);
    let spacing_x = 187.0; // The amount to subtract from the X position for each wall

    for i in 0..9 {
        let x_position = initial_position.x - (i as f32) * spacing_x;
        commands.spawn(SceneBundle {
            scene: wall_model.clone(),
            transform: Transform {
                translation: Vec3::new(x_position, initial_position.y, initial_position.z),
                scale: Vec3::splat(100.0),
               
                ..default()
            },
            ..default()
        })
        .insert(Collider::cuboid(1.0, 100.0, 0.2))
        .insert(RigidBody::Fixed);
    }

   for y in 0..9 {
    let z_position = -365.0 + (y as f32) * spacing_x;
    commands.spawn(SceneBundle {
        scene: wall_model.clone(),
        transform: Transform {
            translation: Vec3::new(107.0, 0.0, z_position),
            scale: Vec3::splat(100.0),
            rotation: Quat::from_rotation_y(89.53),
            ..default()
        },
        ..default()
    })
    .insert(Collider::cuboid(1.0, 100.0, 0.2))
    .insert(RigidBody::Fixed);
   }

  for k in 0..9 {
    let x_position = 320.0 + (k as f32) * spacing_x;
    commands.spawn(SceneBundle {
        scene: wall_model.clone(),
        transform: Transform {
            translation: Vec3::new(x_position, 0.0, 1100.0),
            scale: Vec3::splat(100.0),
            rotation: Quat::from_rotation_y(std::f32::consts::PI),
            ..default()
        },
        ..default()
    })
    .insert(Collider::cuboid(1.0, 100.0, 0.2))
    .insert(RigidBody::Fixed);
  }
    
}