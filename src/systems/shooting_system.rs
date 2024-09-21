use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::{cube::CubeComponent, player::{Player, PlayerFirstPersonCamera}, projectile::BulletTracer};

const MAX_BULLET_DISTANCE: f32 = 1000.0;
const TRACER_LIFETIME: f32 = 0.5;
const TRACER_SPEED: f32 = 200.0;
const TRACER_WIDTH: f32 = 1.0;

pub fn move_towards(current: Vec3, target: Vec3, max_dist_delta: f32) -> Vec3 {
    let a: Vec3 = target - current;
    let magnitude = Vec3::length(a);
    if magnitude <= max_dist_delta || magnitude == 0. {
        return target;
    }
    return current + a / magnitude * max_dist_delta;
}

pub fn shoot_ray(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    camera_query: Query<&Transform, (With<PlayerFirstPersonCamera>, Without<Player>)>,
    rapier_context: Res<RapierContext>,
    cube_query: Query<&CubeComponent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        let player_transform = player_query.single();
        let camera_transform = camera_query.single();

        // Combine player and camera transformations
        let combined_transform = Transform::from_matrix(player_transform.compute_matrix() * camera_transform.compute_matrix());
        
        let ray_origin = combined_transform.translation;
        let ray_direction = combined_transform.forward();

        let hit = rapier_context.cast_ray(
            ray_origin,
            *ray_direction,
            MAX_BULLET_DISTANCE,
            true,
            QueryFilter::default(),
        );

        let (end_position, hit_entity) = if let Some((entity, intersection)) = hit {
            (ray_origin + ray_direction * intersection, Some(entity))
        } else {
            (ray_origin + ray_direction * MAX_BULLET_DISTANCE, None)
        };

        let tracer_length = (end_position - ray_origin).length();
        

        let material = materials.add(StandardMaterial {
            base_color: Color::WHITE,
            emissive: Color::WHITE * 2.0, // Make the tracer glow
            ..Default::default()
        });

        println!("transform {:?}", camera_transform.forward());

        let forward = player_transform.forward();
        let cam_forward = camera_transform.forward();
let forward_x = forward.x;
let forward_y = cam_forward.y; 
let forward_z = forward.z;

        // Spawn bullet tracer
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(TRACER_WIDTH, TRACER_WIDTH, TRACER_WIDTH))),
                material,
                transform: Transform::from_xyz(0., 100000., 0.)
                .looking_to(Vec3::new(forward_x, forward_y,forward_z), Vec3::Y),
                ..default()
            },
            BulletTracer {
                start_position: ray_origin,
                end_position,
                life_time: TRACER_LIFETIME,
                direction: *ray_direction,
            },
        ));

        // Handle hit logic here
        if let Some(entity) = hit_entity {
            if let Ok(_) = cube_query.get(entity) {
                println!("Hit a CubeComponent entity: {:?}", entity);
                // Add your specific logic for hitting a cube here
                // For example, change its color, make it disappear, etc.
            } else {
                println!("Hit an entity, but not a CubeComponent: {:?}", entity);
            }
        } else {
            println!("No entity hit");
        }
    }
}


pub fn update_tracers(
    mut commands: Commands,
    mut tracer_query: Query<(Entity, &mut BulletTracer, &mut Transform, &Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (entity, mut tracer, mut transform, material_handle) in tracer_query.iter_mut() {
        tracer.life_time -= time.delta_seconds();

        if tracer.life_time <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }

        // Fade out the tracer over its lifetime
        if let Some(standard_material) = materials.get_mut(material_handle) {
            let fade_factor = tracer.life_time / TRACER_LIFETIME;
            standard_material.base_color.set_a(fade_factor);
        }

        // Shrink the tracer from the start position
        let new_start = tracer.start_position.lerp(tracer.end_position, 1.0 - (tracer.life_time / TRACER_LIFETIME));
        let new_length = (tracer.end_position - new_start).length();

        transform.translation = (new_start + tracer.end_position) / 2.0;
        transform.scale.z = new_length;
    }
}

// pub fn hitscan_shooting_system(
//     mut commands: Commands,
//     mouse_input: Res<ButtonInput<MouseButton>>,
//     rapier_context: Res<RapierContext>,
//     query: Query<&Transform, With<PlayerFirstPersonCamera>>, // Get the camera transform
// ) {
//     if mouse_input.just_pressed(MouseButton::Left) { // Fire with left mouse button
//         if let Ok(camera_transform) = query.get_single() {
//             // Start the ray from the camera
//             let ray_origin = camera_transform.translation;
//             let ray_direction = camera_transform.forward();

//             // Perform a raycast using Rapier's context
//             if let Some((entity, hit)) = rapier_context.cast_ray(
//                 ray_origin,
//                 *ray_direction,
//                 f32::MAX, // Maximum distance of the ray
//                 true,     // Perform solid raycast (hits on colliders)
//                 QueryFilter::default(),
//             ) {
//                  // Calculate the hit point
//                  let hit_point = ray_origin + *ray_direction * hit;

//                 // Handle hit
//                 println!("Hit entity {:?} at point {:?}", entity, hit_point);

//                 // Optional: Apply damage or destroy hit entity
//                 // commands.entity(entity).despawn(); // Example to despawn hit entity
//             } else {
//                 println!("No hit");
//             }
//         }
//     }
// }