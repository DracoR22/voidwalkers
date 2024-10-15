use bevy::prelude::*;

use crate::{game::weapons::{components::{AK74Component, GlockComponent}, states::{can_shoot_and_decrease_ammo, CurrentWeapon}}, player::components::{Player, PlayerFirstPersonCamera}};

use super::components::{HasMuzzleFlash, MuzzleFlash};

pub fn setup_muzzle_flash(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let muzzle_flash_handle = asset_server.load("models/muzzle-flash.glb#Scene0");

    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: muzzle_flash_handle.clone(),
                    transform: Transform {
                        scale: Vec3::splat(0.3),
                        translation: Vec3::new(90.2, 8.85, -200.0),
                        ..default()
                    },
                      visibility: Visibility::Hidden, 
                    ..default()
                },
                MuzzleFlash {
                    timer: Timer::from_seconds(0.05, TimerMode::Once),
                    is_active: false,
                    frames_visible: 0
                },
            ));
        });
    }
}


pub fn update_muzzle_flash(
    mut commands: Commands,
    time: Res<Time>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut muzzle_flash_query: Query<(Entity, &mut MuzzleFlash, &mut Visibility, &mut Transform)>,
    camera_query: Query<&Transform, (With<PlayerFirstPersonCamera>, Without<MuzzleFlash>)>,
    weapon_state: Res<State<CurrentWeapon>>,
    mut glock_query: Query<&mut GlockComponent>,
    mut ak74_query: Query<&mut AK74Component>,
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
        let can_shoot = can_shoot_and_decrease_ammo(weapon_state.get(), &mut glock_query, &mut ak74_query);
       
       if can_shoot {
        for (_, mut muzzle_flash, mut visibility, mut transform) in muzzle_flash_query.iter_mut() {
            // Reset the muzzle flash timer
            muzzle_flash.timer.reset();
            muzzle_flash.is_active = true;
            muzzle_flash.frames_visible = 0;
            *visibility = Visibility::Visible;
            println!("Updated muzzle flash");
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
    }

    // Update visibility based on the muzzle flash timer
    for (_, mut muzzle_flash, mut visibility, _) in muzzle_flash_query.iter_mut() {
       if muzzle_flash.is_active {
        muzzle_flash.frames_visible += 1;
        if muzzle_flash.timer.tick(time.delta()).just_finished() || muzzle_flash.frames_visible >= 10 {
       
            *visibility = Visibility::Hidden;
            muzzle_flash.is_active = false;
            muzzle_flash.frames_visible = 0;
        }
       }
    }
}