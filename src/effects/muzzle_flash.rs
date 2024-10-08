use bevy::prelude::*;

use crate::{player::components::{Player, PlayerFirstPersonCamera}};

use super::components::{HasMuzzleFlash, MuzzleFlash};

pub fn spawn_muzzle_flash(
    mut commands: Commands,
    player_query: Query<(Entity, Option<&HasMuzzleFlash>), With<Player>>,
    asset_server: Res<AssetServer>
) {
  if let Ok((player_entity, has_flash)) = player_query.get_single() {
    if has_flash.is_none() {
        commands.entity(player_entity).with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: asset_server.load("models/muzzle-flash.glb#Scene0"),
                    transform: Transform {
                        scale: Vec3::splat(0.3), 
                        translation: Vec3::new(90.2, 8.85, -200.0), 
                        // rotation: Quat::from_rotation_y(std::f32::consts::PI), 
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    ..default()
                },
                MuzzleFlash {
                        timer: Timer::from_seconds(0.02, TimerMode::Once),
                 },
            ));
        });

        // Add the HasMuzzleFlash component to prevent future spawns
        commands.entity(player_entity).insert(HasMuzzleFlash);
    }
  }
}

pub fn update_muzzle_flash(
    mut commands: Commands,
    time: Res<Time>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut muzzle_flash_query: Query<(Entity, &mut MuzzleFlash, &mut Visibility, &mut Transform)>,
    camera_query: Query<&Transform, (With<PlayerFirstPersonCamera>, Without<MuzzleFlash>)>,
) {
    // Get the camera transform
    let camera_transform = if let Ok(transform) = camera_query.get_single() {
        transform
    } else {
        return;
    };

    
    let forward_vector = camera_transform.forward();
    let camera_pitch = forward_vector.y;

    // Check if the left mouse button is pressed
    if mouse_input.just_pressed(MouseButton::Left) {
        for (_, mut muzzle_flash, mut visibility, mut transform) in muzzle_flash_query.iter_mut() {
            // Reset the muzzle flash timer
            muzzle_flash.timer.reset();
            *visibility = Visibility::Visible;

            let mut new_y_pos = 72.0;
            let mut new_z_pos = -200.50001;
            
            println!("Y PITCH: {:?}", camera_pitch);

            // camera looking up
            if camera_pitch > 0.66 {
                println!("Camera is more looking up");
                new_y_pos += camera_pitch * 310.0; 
            } 
            else if camera_pitch > 0.6 {
                println!("Camera is more looking up");
                new_y_pos += camera_pitch * 290.0; 
            } 
            else if camera_pitch > 0.47 {
                println!("Camera is more looking up");
                new_y_pos += camera_pitch * 260.0; 
            } 
            else if camera_pitch > 0.25 {
                println!("Camera is more looking up");
                new_y_pos += camera_pitch * 230.0; 
            } 
            else if camera_pitch > 0.02 {
                println!("Camera is looking up");
                new_y_pos += camera_pitch * 200.0; 
            }
            // camera looking down
            else if camera_pitch < -0.04 {
                println!("Camera is looking down");
                new_y_pos += camera_pitch * 150.0;
                new_z_pos += camera_pitch.abs() * 170.0;
            // neutral camera
            } else {
                println!("Camera is looking forward horizontally");
            }
            
            transform.translation = Vec3::new(70.0, new_y_pos, new_z_pos);

            // rotation
            let pitch_rotation = Quat::from_rotation_x(camera_pitch * 1.5);
            transform.rotation = pitch_rotation;
        }
    }

    // Update visibility based on the muzzle flash timer
    for (_, mut muzzle_flash, mut visibility, _) in muzzle_flash_query.iter_mut() {
        if muzzle_flash.timer.tick(time.delta()).just_finished() {
            *visibility = Visibility::Hidden;
        }
    }
}
