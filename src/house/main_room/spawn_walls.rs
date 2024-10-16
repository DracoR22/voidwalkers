use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{common::entities::EntityType, house::components::WallComponent};

pub fn spawn_walls(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let initial_position = Vec3::new(1700.0, 156.0, -570.0);
    let spacing_x = 180.0 - 0.1; // The amount to subtract from the X position for each wall

    let mesh = mesh_assets.add(Cuboid::new(180.0, 375.0, 5.0));

    let texture_handle: Handle<Image> = asset_server.load("textures/WallPaper_ALB.png");
    let normal_map_texture: Handle<Image> = asset_server.load("textures/WallPaper_NRM.png");
    // let rma_texture: Handle<Image> = asset_server.load("textures/WallPaper_RMA.png");

    let material_handle = material_assets.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        normal_map_texture: Some(normal_map_texture),
        // metallic_roughness_texture: Some(rma_texture),
        ..default()
    });  

    let mut walls_vec: Vec<(Vec3, Quat)> = Vec::new();

     // First row of walls
     for i in 0..9 {
        let x_position = initial_position.x - (i as f32) * spacing_x;
        walls_vec.push((Vec3::new(x_position, initial_position.y, initial_position.z), Quat::IDENTITY));
    }

    // Second row of walls (rotated)
    for y in 0..9 {
        let z_position = -480.0 + (y as f32) * spacing_x;
        walls_vec.push((Vec3::new(720.0, initial_position.y, z_position), Quat::from_rotation_y(89.53)));
    }

    // Third row of walls
    for k in 0..9 {
        let x_position = 320.0 + (k as f32) * spacing_x;
        walls_vec.push((Vec3::new(x_position, initial_position.y, 500.0), Quat::from_rotation_y(std::f32::consts::PI)));
    }

    // Fourth row of walls (rotated)
    for t in 0..9 {
        let z_position = 885.0 - (t as f32) * spacing_x;
        walls_vec.push((Vec3::new(1590.0, initial_position.y, z_position), Quat::from_rotation_y(-89.53)));
    }

    for (translation, rotation) in walls_vec {
        commands.spawn(PbrBundle {
            mesh: mesh.clone(),
            material: material_handle.clone(),
            transform: Transform {
                translation,
                rotation,
                ..default()
            },
            ..default()
        })
        .insert(WallComponent)
        .insert(EntityType::Wall)
        .insert(Collider::cuboid(90.0, 187.5, 2.5)) 
        .insert(RigidBody::Fixed);
    }

//     for i in 0..9 {
//         let x_position = initial_position.x - (i as f32) * spacing_x;
//         commands.spawn(PbrBundle {
//             mesh: mesh.clone(),
//             material: material_handle.clone(),
//             transform: Transform {
//                 translation: Vec3::new(x_position, initial_position.y, initial_position.z),
//                 // scale: Vec3::new(100.0, 120.0, 100.0), 
               
//                 ..default()
//             },
//             ..default()
//         })
//         .insert(Collider::cuboid(90.0, 187.5, 2.5))
//         .insert(RigidBody::Fixed);
//     }

//     for y in 0..9 {
//         let z_position = -480.0 + (y as f32) * spacing_x;
//         commands.spawn(PbrBundle {
//             mesh: mesh.clone(),
//             material: material_handle.clone(),
//             transform: Transform {
//                 translation: Vec3::new(720.0, initial_position.y, z_position),
//                 rotation: Quat::from_rotation_y(89.53), 
               
//                 ..default()
//             },
//             ..default()
//         })
        
//         .insert(Collider::cuboid(1.0, 100.0, 0.5))
//         .insert(RigidBody::Fixed);
//     }


// // third row
//   for k in 0..9 {
//     let x_position = 320.0 + (k as f32) * spacing_x;
//     commands.spawn(PbrBundle {
//         mesh: mesh.clone(),
//         material: material_handle.clone(),
//         transform: Transform {
//             translation: Vec3::new(x_position, initial_position.y, 500.0),
//             rotation: Quat::from_rotation_y(std::f32::consts::PI),
//             ..default()
//         },
//         ..default()
//     })
//     .insert(Collider::cuboid(1.0, 100.0, 0.2))
//     .insert(RigidBody::Fixed);
//   }

// //   // fourth row
//   for t in 0..9 {
//     let z_position = 885.0 - (t as f32) * spacing_x;
//     commands.spawn(PbrBundle {
//         mesh: mesh.clone(),
//         material: material_handle.clone(),
//         transform: Transform {
//             translation: Vec3::new(1590.0, initial_position.y, z_position),
//             rotation: Quat::from_rotation_y(-89.53),
//             ..default()
//         },
//         ..default()
//     })
//     .insert(Collider::cuboid(1.0, 100.0, 0.2))
//     .insert(RigidBody::Fixed);
//   }
}