use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use crate::player::{components::{Player, PlayerFirstPersonCamera}, constants::{PLAYER_JUMP, PLAYER_SPEED}};

use super::spawn::AK74Model;

pub fn player_look_system(
    windows: Query<&Window>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<PlayerFirstPersonCamera>, Without<Player>)>,

    mut mouse_motion: EventReader<MouseMotion>,
) {
    let window = windows.single();
    let mut player_transform = player_query.single_mut();
    let mut camera_transform = camera_query.single_mut();

    let mut total_yaw = 0.0f32;
    let mut total_pitch = 0.0f32;

    for ev in mouse_motion.read() {
        total_yaw -= ev.delta.x * 0.005;
        total_pitch -= ev.delta.y * 0.005;
    }

    // Rotate the player around the Y axis
    player_transform.rotate_y(total_yaw);

    // Update the camera pitch
    let (yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);
    pitch += total_pitch;
    pitch = pitch.clamp(-1.54, 1.54); // Approximately -88 to +88 degrees
    camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
}

pub fn player_movement_system(keyboard_input: Res<ButtonInput<KeyCode>>,  mut query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        let forward = transform.forward();
        let right = transform.right();

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += *forward;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= *forward;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= *right;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += *right;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            direction.y += PLAYER_JUMP
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
        }
    }
}

pub fn update_gun_rotation(
    camera_query: Query<&Transform, With<PlayerFirstPersonCamera>>,
    mut gun_query: Query<&mut Transform, (With<AK74Model>, Without<PlayerFirstPersonCamera>)>,
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