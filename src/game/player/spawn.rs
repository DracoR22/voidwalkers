use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping,
    },
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use crate::game::player::{components::{Player, PlayerFirstPersonCamera}, constants::{GRAVITY_SCALE, PLAYER_SPEED}};

#[derive(Component)]
pub struct AK74Model;

pub fn spawn_player_system(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>, asset_server: Res<AssetServer>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    let player_size = 70.0;
    let capsule_radius = player_size / 4.0;

      // Spawn the player entity
      commands.spawn((
        TransformBundle::from(Transform::from_xyz(1000.0, 50.0, 0.0)),
        VisibilityBundle::default(),
        Player,
    ))
    .insert(RigidBody::Dynamic) // Make the player dynamic
    .insert(Collider::cuboid(player_size / 2.0, player_size / 2.0, player_size / 2.0)) 
    .insert(Velocity {
        linvel: Vec3::new(0., 0.0, 0.), 
        angvel: Vec3::ZERO,
    })
    
    .insert(GravityScale(GRAVITY_SCALE))
    .insert(Damping {
        linear_damping: 0.0, 
        angular_damping: 2.0,
    })    
    .insert(LockedAxes::ROTATION_LOCKED)
    .with_children(|parent| {
        // Spawn the camera as a child of the player
        parent.spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                tonemapping: Tonemapping::TonyMcMapface,
                transform: Transform::from_xyz(0.0, 107.7, 10.5),
                ..default()
            },
            BloomSettings::NATURAL,
            PlayerFirstPersonCamera
        ));
    });
}

pub fn rotate_character(
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    for (mut transform, _) in query.iter_mut() {
        // Rotate the character (asset) around its Y-axis
        transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
    }
}