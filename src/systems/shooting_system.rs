use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{components::{player::PlayerFirstPersonCamera, projectile::Projectile}, resources::projectile_properties::ProjectileProperties};

pub fn setup_shooting(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
  let projectile_mesh = meshes.add(Sphere::new(5.0));
  let projectile_material = materials.add(StandardMaterial {
    base_color: Color::rgb(234.0, 239.0, 44.0),
    ..Default::default()
  });

  commands.insert_resource(ProjectileProperties {
    mesh: projectile_mesh,
    material: projectile_material
  });
}

pub fn shooting_pr_system(mut commands: Commands, keyboard_input: Res<ButtonInput<KeyCode>>, projectile_properties: Res<ProjectileProperties>, query: Query<&Transform, With<PlayerFirstPersonCamera>>, time: Res<Time>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(camera_transform) = query.get_single() {
            let projectile_spawn_point = camera_transform.translation + camera_transform.forward() * 2.0;
            
            // Spawn the projectile
            commands.spawn(PbrBundle {
                    mesh: projectile_properties.mesh.clone(),
                    material: projectile_properties.material.clone(),
                    transform: Transform::from_translation(projectile_spawn_point),
                    ..default()
                })
                .insert(Projectile {
                    speed: 20.0,
                    lifetime: Timer::from_seconds(5.0, TimerMode::Once),
                })
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(0.1))
                .insert(Velocity::linear(camera_transform.forward() * 20.0)); // Assign velocity
        }
    }
}

pub fn update_projectiles(
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut Projectile, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut projectile, mut transform) in projectiles.iter_mut() {
        projectile.lifetime.tick(time.delta());
        if projectile.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn projectile_collision_system(
    mut commands: Commands,
    projectiles: Query<Entity, With<Projectile>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if projectiles.contains(*e1) {
                    commands.entity(*e1).despawn();
                }
                if projectiles.contains(*e2) {
                    commands.entity(*e2).despawn();
                }
            }
            _ => {}
        }
    }
}