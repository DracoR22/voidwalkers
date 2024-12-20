use bevy::{
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping, Skybox,
    }, input::mouse::{MouseButtonInput, MouseMotion, MouseWheel}, prelude::*
};
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_rapier3d::prelude::*;

use crate::{common::commands::{action_from_input, Action}, game::player::{components::{Player, PlayerFirstPersonCamera}, constants::{GRAVITY_SCALE, PLAYER_SPEED}}};

#[derive(Component)]
pub struct AK74Model;

pub fn spawn_player_system(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>, asset_server: Res<AssetServer>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    let player_size = 100.0;

    // let skybox_handle = asset_server.load(CUBEMAPS[0].0);

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
                projection: PerspectiveProjection {
                    far: 10000.0, // Set the far plane to a large value to render distant objects and sky
                    ..default()
                }.into(),
                tonemapping: Tonemapping::TonyMcMapface,
                transform: Transform::from_xyz(0.0, 107.7, 10.5),
                ..default()
            },
            BloomSettings::NATURAL,
            PlayerFirstPersonCamera,
            AtmosphereCamera::default()
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

pub fn disable_player_physics_system(mut commands: Commands,  query: Query<(Entity, Option<&RigidBody>, Option<&Collider>), With<Player>>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    let actions = action_from_input(&keyboard_input);
   if let Ok ((player, rigid_body, collider)) = query.get_single() {
    for action in actions {
        match action {
         Action::TogglePhysics => {
            
                // Remove physics components
                println!("EDIT MOODE");
                commands.entity(player)
                .remove::<RigidBody>() 
                .remove::<Collider>()  
                .remove::<GravityScale>() 
                .remove::<Velocity>()
                .remove::<Damping>()  
                .remove::<LockedAxes>(); 
         },
         _ => ()
        }
     }
   }
}

pub fn enable_player_physics_system(mut commands: Commands, query: Query<Entity, With<Player>>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    let actions = action_from_input(&keyboard_input);
    if let Ok (player) = query.get_single() {
     for action in actions {
         match action {
          Action::TogglePhysics => {
                 println!("GAME MODE");
                 // Add physics components back
                 commands.entity(player)
                 .insert(RigidBody::Dynamic) 
                 .insert(Collider::cuboid(70.0 / 2.0, 70.0 / 2.0, 70.0 / 2.0)) 
                 .insert(GravityScale(GRAVITY_SCALE)) 
                 .insert(Velocity {
                     linvel: Vec3::new(0., 0.0, 0.), 
                     angvel: Vec3::ZERO,
                 })
                 .insert(Damping {
                     linear_damping: 0.0, 
                     angular_damping: 2.0,
                 }) 
                 .insert(LockedAxes::ROTATION_LOCKED); 
          },
          _ => ()
         }
      }
    }
}
