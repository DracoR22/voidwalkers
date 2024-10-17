use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::{game::player::components::{Player, PlayerFirstPersonCamera}, game::weapons::{components::{AK74Component, HasAK74}, state::CurrentWeapon}};

pub fn spawn_ak74(  
    mut commands: Commands,
    player_query: Query<(Entity, Option<&HasAK74>), With<Player>>, // Check if the player has the AK74
    asset_server: Res<AssetServer>,
    state: Res<State<CurrentWeapon>>,
) {
   match state.get() {
    CurrentWeapon::AK74 => {
         // Get the player entity and check if they already have the AK74
    if let Ok((player_entity, has_ak74)) = player_query.get_single() {
        if has_ak74.is_none() { 
        println!("SPAWNED AK!");
            commands.entity(player_entity).with_children(|parent| {
                parent.spawn((
                    SceneBundle {
                        scene: asset_server.load("animations/ak74.glb#Scene0"),
                        transform: Transform {
                            scale: Vec3::splat(50.0), 
                             translation: Vec3::new(0.2, 85.5, 0.3), 
                             rotation: Quat::from_rotation_y(std::f32::consts::PI), 
                            ..default()
                        },
                        ..default()
                    },
                    AK74Component {
                        current_ammo: 35,
                        magazine_size: 35,
                        max_ammo: 325,
                        reload_time: 1.0,
                        reload_timer: 0.0
                    }
                ));
            });

            // Add the HasAK74 component to prevent future spawns
            commands.entity(player_entity).insert(HasAK74);
        }
    }
    }
    _ => ()
   }
}

pub fn despawn_ak74(
    mut commands: Commands,
    player_query: Query<(Entity, Option<&HasAK74>), With<Player>>, 
    ak74_query: Query<Entity, With<AK74Component>>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    state: Res<State<CurrentWeapon>>,
    mut next_state: ResMut<NextState<CurrentWeapon>>,
) {
   match state.get() {
    CurrentWeapon::AK74 => {
          // Get the player entity and check if they have the AK74
     if keyboard_input.just_pressed(KeyCode::KeyQ) {
        if let Ok((player_entity, has_ak74)) = player_query.get_single() {
            if has_ak74.is_some() {
                println!("DESPAWNING AK!");

                // Iterate over all AK74 entities and despawn them
                for ak74_entity in ak74_query.iter() {
                    commands.entity(ak74_entity).despawn_recursive(); // Despawn the AK74 entity and its children
                }

                // Spawn Glock after despawning AK-74
                commands.entity(player_entity).remove::<HasAK74>();
                // commands.entity(player_entity).insert(CurrentWeapon::Glock);

                // AK74 switches to glock
                next_state.set(CurrentWeapon::Glock);
            }
        }
 }
    }
    _ => ()
   }
}

pub fn respawn_ak74(
    mut commands: Commands,
    player_query: Query<(Entity, Option<&HasAK74>), With<Player>>, // Check if the player already has the AK74
    mut evr_scroll: EventReader<MouseWheel>,                      // MouseWheel event reader
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    // Check for mouse wheel scroll down
    for event in evr_scroll.read() {
        if event.y < 0.0 { // Scroll down direction
            // Get the player entity and check if they don't already have the AK74
            if let Ok((player_entity, has_ak74)) = player_query.get_single() {
                if has_ak74.is_none() { // Player doesn't have AK-74, respawn it
                    println!("RESPAWNING AK!");

                    // Spawn the AK-74 as a child of the player entity
                    commands.entity(player_entity).with_children(|parent| {
                        parent.spawn((
                            SceneBundle {
                                scene: asset_server.load("animations/ak74.glb#Scene0"),
                                transform: Transform {
                                    scale: Vec3::splat(50.0), 
                                    translation: Vec3::new(0.2, 85.5, 0.3), 
                                    rotation: Quat::from_rotation_y(std::f32::consts::PI), 
                                    ..default()
                                },
                                ..default()
                            },
                            AK74Component {
                                current_ammo: 35,
                                magazine_size: 35,
                                max_ammo: 325,
                                reload_time: 1.0,
                                reload_timer: 0.0
                            },
                        ));
                    });

                    // Add the HasAK74 component to prevent multiple spawns
                    commands.entity(player_entity).insert(HasAK74);
                }
            }
        }
    }
}


pub fn update_gun_rotation(
    camera_query: Query<&Transform, With<PlayerFirstPersonCamera>>,
    mut gun_query: Query<&mut Transform, (With<AK74Component>, Without<PlayerFirstPersonCamera>)>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        if let Ok(mut gun_transform) = gun_query.get_single_mut() {
            // Create a rotation that aligns the gun with the camera
            let gun_rotation = camera_transform.rotation * Quat::from_rotation_y(std::f32::consts::PI);

            // Update the gun's rotation
            gun_transform.rotation = gun_rotation;

            let forward_vec = camera_transform.forward();
            let camera_pitch = forward_vec.y; 

            let dynamic_z_offset = if camera_pitch >= 0.0 {
                15.9 - (camera_pitch * -5.2)  // Move closer when looking up
            } else {
                15.9 + (camera_pitch.abs() * -30.2)  // Move further away when looking down
            };

            // Adjust the gun's position relative to the camera
            gun_transform.translation = camera_transform.translation
                + camera_transform.forward() * dynamic_z_offset // Move it forward 
                + camera_transform.right() * 0.70 // Move it to the right 
                - camera_transform.up() * 1.1 // Move it down 
                + Vec3::new(0.0, -20.3, 0.0); // Additional downward offset
        }
    }
}