use bevy::prelude::*;

use crate::{player::components::{Player, PlayerFirstPersonCamera}, weapons::components::AK74Component};

#[derive(Component)]
pub struct HasAK74;

pub fn spawn_ak74(
    mut commands: Commands,
    player_query: Query<(Entity, Option<&HasAK74>), With<Player>>, // Check if the player has the AK74
    asset_server: Res<AssetServer>
) {
    // Get the player entity and check if they already have the AK74
    if let Ok((player_entity, has_ak74)) = player_query.get_single() {
        if has_ak74.is_none() { // If the player doesn't have the AK74, spawn it
        println!("SPAWNED AK!");
            commands.entity(player_entity).with_children(|parent| {
                parent.spawn((
                    SceneBundle {
                        scene: asset_server.load("animations/ak.glb#Scene0"),
                        transform: Transform {
                            scale: Vec3::splat(50.0), 
                             translation: Vec3::new(0.2, 85.5, 0.3), 
                             rotation: Quat::from_rotation_y(std::f32::consts::PI), 
                            ..default()
                        },
                        ..default()
                    },
                    AK74Component
                ));
            });

            // Add the HasAK74 component to prevent future spawns
            commands.entity(player_entity).insert(HasAK74);
        }
    }
}

pub fn update_gun_rotation(
    camera_query: Query<&Transform, With<PlayerFirstPersonCamera>>,
    mut gun_query: Query<&mut Transform, (With<AK74Component>, Without<PlayerFirstPersonCamera>)>,
) {
    let camera_transform = camera_query.single();
    let mut gun_transform = gun_query.single_mut();

    // Create a rotation that aligns the gun with the camera
    let gun_rotation = camera_transform.rotation * Quat::from_rotation_y(std::f32::consts::PI);

    // Update the gun's rotation
    gun_transform.rotation = gun_rotation;

    // Adjust the gun's position relative to the camera
    gun_transform.translation = camera_transform.translation 
        + camera_transform.forward() * 0.3 // Move it forward (reduced from 0.5)
        + camera_transform.right() * 0.15 // Move it to the right (reduced from 0.3)
        - camera_transform.up() * 0.1 // Move it down (reduced from 0.2)
        + Vec3::new(0.0, -20.3, 0.0); // Additional downward offset
}