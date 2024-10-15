use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_hanabi::prelude::*;
use bevy::prelude::AlphaMode;

#[derive(Resource)]
pub struct WeaponFireTimer(pub Timer);

use crate::{cubes::components::CubeComponent, effects::blood_decal::spawn_blood, game::weapons::{components::{AK74Component, GlockComponent}, resources::AK74Timer, states::{can_shoot_and_decrease_ammo, CurrentWeapon}}, player::{components::{BulletTracer, Player, PlayerFirstPersonCamera}, constants::{MAX_BULLET_DISTANCE, TRACER_LIFETIME, TRACER_WIDTH}}};

pub fn shoot_ray(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    player_query: Query<&Transform, With<Player>>,
    camera_query: Query<&Transform, (With<PlayerFirstPersonCamera>, Without<Player>)>,
    mut glock_query: Query<&mut GlockComponent>,
    mut ak74_query: Query<&mut AK74Component>,
    rapier_context: Res<RapierContext>,
    cube_query: Query<(&CubeComponent, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    effects: ResMut<Assets<EffectAsset>>,
    asset_server: Res<AssetServer>,
    weapon_state: Res<State<CurrentWeapon>>,
    mut weapon_fire_timer: ResMut<WeaponFireTimer>,
    time: Res<Time>,
) {
    weapon_fire_timer.0.tick(time.delta());
    if mouse_input.pressed(MouseButton::Left) && weapon_fire_timer.0.finished() {
        let player_transform = player_query.single();
        let camera_transform = camera_query.single();

        // Combine player and camera transformations
        let combined_transform = Transform::from_matrix(player_transform.compute_matrix() * camera_transform.compute_matrix());
        
        let ray_origin = combined_transform.translation;
        let ray_direction = combined_transform.forward();

        let can_shoot = can_shoot_and_decrease_ammo(weapon_state.get(), &mut glock_query, &mut ak74_query);

        // handle ammo logic
       
          if can_shoot {

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
            println!("Tracer Start Position (World Coordinates): {:?}", ray_origin);
    
            let material = materials.add(StandardMaterial {
                base_color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                alpha_mode: AlphaMode::Mask(0.0),
                
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
                    .looking_to(Vec3::new(forward_x, forward_y, forward_z), Vec3::Y),
                    // visibility: Visibility::Hidden,
                    ..default()
                },
                BulletTracer {
                    start_position: ray_origin,
                    end_position,
                    life_time: TRACER_LIFETIME,
                    direction: *ray_direction,
                },
            ))
            .insert(Collider::capsule_y(TRACER_WIDTH / 2.0, TRACER_WIDTH));

          // Handle hit logic
          if let Some(entity) = hit_entity {
            if let Ok((_, transform)) = cube_query.get(entity) {
                println!("Hit a CubeComponent entity: {:?}", entity);

                spawn_blood(commands, effects, 0.0, 0.0, 0.0, Some(entity), asset_server);
            } else {
                println!("Hit an entity, but not a CubeComponent: {:?}", entity);
            }
        } else {
            println!("No entity hit");
        }
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
