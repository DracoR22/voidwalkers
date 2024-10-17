use bevy::prelude::AlphaMode;
use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    common::entities::EntityType,
    cubes::components::CubeComponent,
    effects::{blood_decal::spawn_blood, bullet_hole::spawn_plaster_bullethole},
    game::{
        player::{
            components::{BulletTracer, Player, PlayerFirstPersonCamera},
            constants::{MAX_BULLET_DISTANCE, TRACER_LIFETIME, TRACER_WIDTH},
        },
        weapons::{
            common::{can_shoot_and_decrease_ammo, WeaponFireTimer},
            components::{AK74Component, GlockComponent},
            resources::{AK74Timer, GlockTimer},
            state::CurrentWeapon,
        },
    },
};

pub fn shoot_ray(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    player_query: Query<&Transform, With<Player>>,
    camera_query: Query<&Transform, (With<PlayerFirstPersonCamera>, Without<Player>)>,
    glock_query: Query<&mut GlockComponent>,
    ak74_query: Query<&mut AK74Component>,
    rapier_context: Res<RapierContext>,
    cube_query: Query<(&CubeComponent, &Transform)>,
    entity_query: Query<(&EntityType, &Transform)>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    effects: ResMut<Assets<EffectAsset>>,
    asset_server: Res<AssetServer>,
    weapon_state: Res<State<CurrentWeapon>>,
    mut weapon_fire_timer: ResMut<WeaponFireTimer>,
    // mut ak74_fire_timer: ResMut<AK74Timer>,
    time: Res<Time>,
) {
    let player_transform = player_query.single();
    let camera_transform = camera_query.single();

    // Combine player and camera transformations
    let combined_transform = Transform::from_matrix(
        player_transform.compute_matrix() * camera_transform.compute_matrix(),
    );

    let ray_origin = combined_transform.translation;
    let ray_direction = combined_transform.forward();

    match weapon_state.get() {
        CurrentWeapon::Glock => {
            if mouse_input.just_pressed(MouseButton::Left) {
                let can_shoot = can_shoot_and_decrease_ammo(glock_query);

                if can_shoot {
                    handle_shoot(
                        commands,
                        ray_origin,
                        *ray_direction,
                        &rapier_context,
                        &cube_query,
                        meshes,
                        materials,
                        effects,
                        asset_server,
                        entity_query,
                        player_query
                    );
                }
            }
        }

        CurrentWeapon::AK74 => {
            weapon_fire_timer.0.tick(time.delta());
            if mouse_input.pressed(MouseButton::Left) && weapon_fire_timer.0.finished() {
                let can_shoot = can_shoot_and_decrease_ammo(ak74_query);

                if can_shoot {
                    handle_shoot(
                        commands,
                        ray_origin,
                        *ray_direction,
                        &rapier_context,
                        &cube_query,
                        meshes,
                        materials,
                        effects,
                        asset_server,
                        entity_query,
                        player_query
                    );
                }
            }
        }

        CurrentWeapon::None => {}
    }
}

fn handle_shoot(
    mut commands: Commands,
    ray_origin: Vec3,
    ray_direction: Vec3,
    rapier_context: &Res<RapierContext>,
    cube_query: &Query<(&CubeComponent, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    effects: ResMut<Assets<EffectAsset>>,
    asset_server: Res<AssetServer>,
    entity_query: Query<(&EntityType, &Transform)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let hit = rapier_context.cast_ray(
        ray_origin,
        ray_direction,
        MAX_BULLET_DISTANCE,
        true,
        QueryFilter::default(),
    );

    let (end_position, hit_entity) = if let Some((entity, intersection)) = hit {
        (ray_origin + ray_direction * intersection, Some(entity))
    } else {
        (ray_origin + ray_direction * MAX_BULLET_DISTANCE, None)
    };

    // Spawn bullet tracer
    let tracer_material = materials.add(StandardMaterial {
        base_color: Color::rgba(1.0, 1.0, 1.0, 0.0),
        alpha_mode: AlphaMode::Mask(0.0),
        ..Default::default()
    });

    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(
                    TRACER_WIDTH,
                    TRACER_WIDTH,
                    TRACER_WIDTH,
                ))),
                material: tracer_material,
                transform: Transform::from_translation(ray_origin)
                    .looking_to(ray_direction, Vec3::Y),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            BulletTracer {
                start_position: ray_origin,
                end_position,
                life_time: TRACER_LIFETIME,
                direction: ray_direction,
            },
        ))
        .insert(Collider::capsule_y(TRACER_WIDTH / 2.0, TRACER_WIDTH));

    // Handle hit logic
    if let Some(entity) = hit_entity {
        if let Some((entity_type, transform)) = entity_query.get(entity).ok() {
            match entity_type {
                EntityType::Cube => {
                    println!("Hit a CubeComponent entity: {:?}", entity);
                    spawn_blood(commands, effects, 0.0, 0.0, 0.0, Some(entity), asset_server);
                }

                EntityType::Floor => {
                    spawn_plaster_bullethole(
                        commands,
                        asset_server,
                        materials,
                        meshes,
                        end_position,
                        Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                    );
                    println!("Hit a floor component: {:?}", entity);
                }

                EntityType::Wall => {
                    let player_rotation = player_query.single().rotation;
                    let forward_direction = player_rotation * Vec3::Z;

                    // I placed each bullethole one unit forward to the player direction
                    let adjusted_position = end_position - forward_direction.normalize() * -1.0;

                    spawn_plaster_bullethole(
                        commands,
                        asset_server,
                        materials,
                        meshes,
                        Vec3::new(adjusted_position.x , end_position.y, adjusted_position.z),
                        player_query.single().rotation,
                    );
                    println!("Hit a wall component: {:?}", entity);
                }
            }
        }
    }
}

pub fn update_tracers(
    mut commands: Commands,
    mut tracer_query: Query<(
        Entity,
        &mut BulletTracer,
        &mut Transform,
        &Handle<StandardMaterial>,
    )>,
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
        let new_start = tracer.start_position.lerp(
            tracer.end_position,
            1.0 - (tracer.life_time / TRACER_LIFETIME),
        );
        let new_length = (tracer.end_position - new_start).length();

        transform.translation = (new_start + tracer.end_position) / 2.0;
        transform.scale.z = new_length;
    }
}
