use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{common::entities::EntityType, game::map::house::components::WallComponent};

pub fn spawn_walls(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let initial_position = Vec3::new(1700.0, 156.0, -570.0);
    let spacing_x = 180.0 - 0.1; // The amount to subtract from the X position for each wall

    let wall_mesh = mesh_assets.add(Cuboid::new(180.0, 375.0, 5.0));

    let short_wall_mesh = mesh_assets.add(Cuboid::new(170.0, 150.0, 5.0));
    let short_wall_mesh_high = mesh_assets.add(Cuboid::new(170.0, 70.0, 5.0));

    let wall_door_mesh = mesh_assets.add(Cuboid::new(150.0, 55.0, 5.0));

    let texture_handle: Handle<Image> = asset_server.load("textures/Wall2_ALB.png");
    let normal_map_texture: Handle<Image> = asset_server.load("textures/Wall2_NRM.png");
    // let rma_texture: Handle<Image> = asset_server.load("textures/WallPaper_RMA.png");

    let material_handle = material_assets.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        normal_map_texture: Some(normal_map_texture),
        // metallic_roughness_texture: Some(rma_texture),
        ..default()
    });  

    let mut walls_vec: Vec<(Vec3, Quat, Handle<Mesh>)> = Vec::new();

     // First row of walls
    //  for i in 0..9 {
    //     let x_position = initial_position.x - (i as f32) * spacing_x;
    //     walls_vec.push((Vec3::new(x_position, initial_position.y, initial_position.z), Quat::IDENTITY, wall_mesh.clone()));
    // }

    for i in 0..4 {
        let x_position = initial_position.x - 143.0 - (i as f32) * spacing_x;
        walls_vec.push((Vec3::new(x_position, initial_position.y, initial_position.z), Quat::IDENTITY, wall_mesh.clone()));
    }

     /* window gap */
    walls_vec.push((Vec3::new(680.0, initial_position.y, initial_position.z), Quat::IDENTITY, wall_mesh.clone()));

    // lower portion
    walls_vec.push((Vec3::new(850.0, initial_position.y - 100.0, initial_position.z), Quat::IDENTITY, short_wall_mesh.clone()));

    // higher portion
    walls_vec.push((Vec3::new(850.0, initial_position.y + 152.0, initial_position.z), Quat::IDENTITY, short_wall_mesh_high.clone()));

    // Second row of walls where door is located
    for y in 0..2 {
        let z_position = -480.0 + (y as f32) * spacing_x;
        walls_vec.push((Vec3::new(720.0, initial_position.y, z_position), Quat::from_rotation_y(89.53), wall_mesh.clone()));
    }

    // door wall
    walls_vec.push((Vec3::new(719.0, 314.0, -135.0), Quat::from_rotation_y(89.53), wall_door_mesh.clone()));

    // second row of walls after the door wall
    for y in 0..4 {
        let z_position = 30.0 + (y as f32) * spacing_x;
        walls_vec.push((Vec3::new(719.0, initial_position.y, z_position), Quat::from_rotation_y(89.53), wall_mesh.clone()));
    }


    // Third row of walls
    for k in 0..9 {
        let x_position = 320.0 + (k as f32) * spacing_x;
        walls_vec.push((Vec3::new(x_position, initial_position.y, 500.0), Quat::from_rotation_y(std::f32::consts::PI), wall_mesh.clone()));
    }

    // Fourth row of walls (rotated)
    for t in 0..9 {
        let z_position = 885.0 - (t as f32) * spacing_x;
        walls_vec.push((Vec3::new(1460.0, initial_position.y, z_position), Quat::from_rotation_y(-89.53), wall_mesh.clone()));
    }

    for (translation, rotation, mesh) in walls_vec {
        commands.spawn(PbrBundle {
            mesh: mesh,
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
}