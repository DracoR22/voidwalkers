use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use crate::components::player::{Player, PlayerFirstPersonCamera};

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_JUMP: f32 = 20.0;
pub const GRAVITY_SCALE: f32 = 7.0;

pub fn spawn_player_system(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>) {
    let player_size = 50.0;
    let eye_height = player_size * 0.8; // Position the camera near the top of the player

    let mesh = mesh_assets.add(Cuboid::new(player_size, player_size,player_size)); 

    commands.spawn(PbrBundle {
        mesh: mesh.clone(),
        transform: Transform::from_translation(Vec3::new(500.0, 50.0, 0.0)), // Start above ground
        ..default()
    })
    .insert(RigidBody::Dynamic) // Make the player dynamic
    .insert(Collider::cuboid(player_size / 2.0, player_size / 2.0, player_size / 2.0)) // Set collider
    .insert(GravityScale(GRAVITY_SCALE))
    .insert(Friction {
        coefficient: 0.5, // Adjust the friction coefficient (lower values for less sliding)
        combine_rule: CoefficientCombineRule::Multiply, // You can use different combination rules
    })
    .insert(Damping {
        linear_damping: 0.5, // Adjust linear damping to slow movement after collisions
        angular_damping: 2.0, // Prevents unwanted rotation
    })
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(Player) // Add the Player component
    .with_children(|parent| {
        parent.spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, eye_height, 0.0),
                ..default()
            },
            PlayerFirstPersonCamera
        ));
    });
}

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