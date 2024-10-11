use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    prelude::*,
};

use crate::{player::{components::{Player, PlayerFirstPersonCamera}, constants::{PLAYER_JUMP, PLAYER_SPEED}}, weapons::components::AK74Component};

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

        let mut speed = PLAYER_SPEED;

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
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            speed *= 2.0;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            transform.translation += direction * speed * time.delta_seconds();
        }
    }
}