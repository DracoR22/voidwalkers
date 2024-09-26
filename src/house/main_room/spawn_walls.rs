use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_walls(mut commands: Commands, asset_server: Res<AssetServer>) {
    let wall_model: Handle<Scene> = asset_server.load("models/wall1.glb#Scene0");
    let wall_model_alt: Handle<Scene> = asset_server.load("models/wall2.glb#Scene0");

    let initial_position = Vec3::new(1577.0, 0.0, -570.0);
    let spacing_x = 185.0; // The amount to subtract from the X position for each wall

    // first row
    for i in 0..9 {
        let x_position = initial_position.x - (i as f32) * spacing_x;
        commands.spawn(SceneBundle {
            scene: wall_model_alt.clone(),
            transform: Transform {
                translation: Vec3::new(x_position, initial_position.y, initial_position.z),
                scale: Vec3::splat(100.0),
               
                ..default()
            },
            ..default()
        })
        .insert(Collider::cuboid(1.0, 100.0, 0.2))
        .insert(RigidBody::Fixed);
    }

    // second row
   for y in 0..9 {
    let z_position = -365.0 + (y as f32) * spacing_x;
    commands.spawn(SceneBundle {
        scene: wall_model_alt.clone(),
        transform: Transform {
            translation: Vec3::new(107.0, 0.0, z_position),
            scale: Vec3::splat(100.0),
            rotation: Quat::from_rotation_y(89.53),
            ..default()
        },
        ..default()
    })
    .insert(Collider::cuboid(1.0, 100.0, 0.2))
    .insert(RigidBody::Fixed);
   }

   // third row
  for k in 0..9 {
    let x_position = 320.0 + (k as f32) * spacing_x;
    commands.spawn(SceneBundle {
        scene: wall_model_alt.clone(),
        transform: Transform {
            translation: Vec3::new(x_position, 0.0, 500.0),
            scale: Vec3::splat(100.0),
            rotation: Quat::from_rotation_y(std::f32::consts::PI),
            ..default()
        },
        ..default()
    })
    .insert(Collider::cuboid(1.0, 100.0, 0.2))
    .insert(RigidBody::Fixed);
  }

  // fourth row
  for t in 0..9 {
    let z_position = 885.0 - (t as f32) * spacing_x;
    commands.spawn(SceneBundle {
        scene: wall_model.clone(),
        transform: Transform {
            translation: Vec3::new(1790.0, 0.0, z_position),
            scale: Vec3::splat(100.0),
            rotation: Quat::from_rotation_y(-89.53),
            ..default()
        },
        ..default()
    })
    .insert(Collider::cuboid(1.0, 100.0, 0.2))
    .insert(RigidBody::Fixed);
  }
}