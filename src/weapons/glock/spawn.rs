use bevy::prelude::*;

use crate::{player::components::{Player, PlayerFirstPersonCamera}, weapons::{components::{GlockComponent, HasGlock}, resources::CurrentWeapon}};

pub fn spawn_glock(
    mut commands: Commands,
    player_query: Query<(Entity, Option<&HasGlock>), With<Player>>,
    asset_server: Res<AssetServer>,
    state: Res<State<CurrentWeapon>>
) {
  match state.get() {
    CurrentWeapon::Glock => {
        // Get the player entity and check if they already have the Glock
  if let Ok((player_entity, has_glock)) = player_query.get_single() {
    if has_glock.is_none() {
        println!("SPAWNED GLOCK!");
        commands.entity(player_entity).with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: asset_server.load("animations/glock.glb#Scene0"),
                    transform: Transform {
                        scale: Vec3::splat(30.0), 
                        translation: Vec3::new(0.2, 85.0, 7.0), 
                        rotation: Quat::from_rotation_y(std::f32::consts::PI), 
                        ..default()
                    },
                    ..default()
                },
                GlockComponent,
            ));
        });

        // Add the HasGlock component to prevent future spawns
        commands.entity(player_entity).insert(HasGlock);
    }
}
    }
    _ => ()
  }
}

pub fn despawn_glock(
    mut commands: Commands,
    player_query: Query<(Entity, Option<&HasGlock>), With<Player>>,
    glock_query: Query<Entity, With<GlockComponent>>, 
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<CurrentWeapon>>,
    mut next_state: ResMut<NextState<CurrentWeapon>>,
) {
  match state.get() {
    CurrentWeapon::Glock => {
        if keyboard_input.just_pressed(KeyCode::KeyQ) {
            if let Ok((player_entity, has_glock)) = player_query.get_single() {
                if let Some(_) = has_glock {
                    println!("DESPAWNING GLOCK!");
        
                    // Iterate over all Glock entities and despawn them
                    for glock_entity in glock_query.iter() {
                        commands.entity(glock_entity).despawn_recursive(); // Despawn the Glock entity and its children
                    }
        
                    // Remove the HasGlock component from the player
                    commands.entity(player_entity).remove::<HasGlock>();
                    // commands.entity(player_entity).insert(CurrentWeapon::AK74);

                    next_state.set(CurrentWeapon::AK74);
                }
            }
           }
    }
    _ => ()
  }
}

pub fn update_gun_rotation(
    camera_query: Query<&Transform, With<PlayerFirstPersonCamera>>,
    mut gun_query: Query<&mut Transform, (With<GlockComponent>, Without<PlayerFirstPersonCamera>)>,
) {
    // Ensure the camera exists before proceeding
    if let Ok(camera_transform) = camera_query.get_single() {
        // Ensure the gun exists before trying to modify its transform
        if let Ok(mut gun_transform) = gun_query.get_single_mut() {
            // Create a rotation that aligns the gun with the camera
            let gun_rotation = camera_transform.rotation * Quat::from_rotation_y(std::f32::consts::PI);

            // Update the gun's rotation
            gun_transform.rotation = gun_rotation;

             // Get the pitch angle (rotation around X-axis) from the camera's rotation
             let forward_vec = camera_transform.forward();
             let camera_pitch = forward_vec.y; 

            //  let dynamic_z_offset = 0.3 - (camera_pitch * -5.2);

            let dynamic_z_offset = if camera_pitch >= 0.0 {
                2.9 - (camera_pitch * -5.2)  // Move closer when looking up
            } else {
                2.9 + (camera_pitch.abs() * -3.5)  // Move further away when looking down
            };

            // Adjust the gun's position relative to the camera
            gun_transform.translation = camera_transform.translation
                + camera_transform.forward() * dynamic_z_offset // Move it forward (reduced from 0.5)
                + camera_transform.right() * 0.15 // Move it to the right (reduced from 0.3)
                - camera_transform.up() * 0.1 // Move it down (reduced from 0.2)
                + Vec3::new(0.0, -4.3, 0.0); // Additional downward offset
        }
    }
}