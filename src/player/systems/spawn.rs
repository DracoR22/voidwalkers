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

use crate::player::{components::{Player, PlayerFirstPersonCamera}, constants::GRAVITY_SCALE, resources::Animations};

#[derive(Component)]
pub struct AK74Model;

pub fn spawn_player_system(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>, asset_server: Res<AssetServer>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    let player_size = 50.0;
    let eye_height = player_size * 0.8; // Position the camera near the top of the player

    let mesh = mesh_assets.add(Cuboid::new(player_size, player_size,player_size)); 

    commands.insert_resource(Animations(vec![
        asset_server.load("animations/ak.glb#Animation0"), // add more animations
        asset_server.load("animations/ak.glb#Animation1"),
        asset_server.load("animations/ak.glb#Animation2"),
        asset_server.load("animations/ak.glb#Animation3"),
        asset_server.load("animations/ak.glb#Animation4"),
        asset_server.load("animations/ak.glb#Animation5"),
        asset_server.load("animations/ak.glb#Animation6"),
        asset_server.load("animations/ak.glb#Animation7")
    ]));

      // Spawn the player entity
      commands.spawn((
        TransformBundle::from(Transform::from_xyz(500.0, 50.0, 0.0)),
        VisibilityBundle::default(),
        Player,
    ))
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

        // Spawn the model as a separate child of the player
        parent.spawn((
            SceneBundle {
                scene: asset_server.load("animations/ak.glb#Scene0"),
                transform: Transform {
                    scale: Vec3::splat(50.0), // Adjust scale if needed
                    // translation: Vec3::new(0.2, -0.5, 0.3), 
                    // rotation: Quat::from_rotation_y(std::f32::consts::PI), 
                    ..default()
                },
                ..default()
            },
            AK74Model, // New component to identify the model
        ));
    });

    //  commands.spawn(SceneBundle {
    //     scene: asset_server.load("animations/saiga.glb#Scene0"),
    //     transform: Transform {
    //         translation: Vec3::new(500.0, 50.0, 0.0),
    //         scale: Vec3::splat(50.0), 
    //         ..default()   
    //     },
    //     ..default()
        
    // }) 
    // .insert(Player) // Add the Player component
    // .with_children(|parent| {
    //     parent.spawn((
    //         Camera3dBundle {
    //             transform: Transform::from_xyz(0.0,0.9, 0.5),
    //             ..default()
    //         },
    //         PlayerFirstPersonCamera
    //     ));
    // });
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