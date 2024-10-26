use bevy::prelude::*;
use bevy_rapier3d::parry::transformation::utils::transform;

use crate::{common::states::CurrentWeapon, game::{player::components::{Player, PlayerFirstPersonCamera}, weapons::glock::{components::{GlockComponent, HasGlock}, GLOCK_MAGAZINE_SIZE, MAX_GLOCK_AMMO}}};

pub const HIDE_OFFSET: f32 = -1000.0;

pub fn spawn_glock(
    mut commands: Commands,
    player_query: Query<(Entity, Option<&HasGlock>), With<Player>>, 
    glock_query: Query<Entity, With<GlockComponent>>,             
    asset_server: Res<AssetServer>,
    state: Res<State<CurrentWeapon>>
) {
    match state.get() {
        CurrentWeapon::Glock => {
            if let Ok((player_entity, has_glock)) = player_query.get_single() {
                if has_glock.is_none() {
                    // If there is no Glock yet, we spawn a new one
                    if let Ok(glock_entity) = glock_query.get_single() {
                        // Glock already exists, so we reattach it to the player
                        println!("Reattaching Glock!");
                        commands.entity(player_entity).add_child(glock_entity);
                    } else {
                        // Spawn a new Glock and attach it to the player
                        println!("SPAWNED GLOCK!");
                        commands.entity(player_entity).with_children(|parent| {
                            parent.spawn((
                                SceneBundle {
                                    scene: asset_server.load("animations/glock.glb#Scene0"),
                                    transform: Transform {
                                        scale: Vec3::splat(30.0),
                                        translation: Vec3::new(0.2, 85.0, 7.0),
                                        rotation: Quat::from_rotation_y(std::f32::consts::PI),
                                        ..default()
                                    },
                                    ..default()
                                },
                                GlockComponent {
                                    current_ammo: GLOCK_MAGAZINE_SIZE,
                                    magazine_size: GLOCK_MAGAZINE_SIZE,
                                    max_ammo: MAX_GLOCK_AMMO,
                                    timer: 0.6,
                                },
                            ));
                        });
                    }

                    // Add the HasGlock component to the player to prevent future spawns
                    commands.entity(player_entity).insert(HasGlock);
                }
            }
        }
        _ => (),
    }
}


pub fn despawn_glock(
    mut commands: Commands,
    mut param_set: ParamSet<(
        Query<(Entity, Option<&HasGlock>, &Transform), With<Player>>,
        Query<(Entity, &mut Transform), With<GlockComponent>>,    
    )>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<CurrentWeapon>>,
    mut next_state: ResMut<NextState<CurrentWeapon>>,
) {
  match state.get() {
    CurrentWeapon::Glock => {
        if keyboard_input.just_pressed(KeyCode::KeyQ) {
           if let Ok((player_entity, has_glock, player_transform)) = param_set.p0().get_single() {
             if let Some(_) = has_glock {
                for (glock_entity, mut glock_transform) in param_set.p1().iter_mut() {
                    commands.entity(player_entity).remove::<HasGlock>();
                    commands.entity(glock_entity).remove_parent();
                 }  

                 next_state.set(CurrentWeapon::AK74);
              }
           }
           }
    }
    _ => ()
  }
}

pub fn update_gun_rotation(
    camera_query: Query<&Transform, With<PlayerFirstPersonCamera>>,
    mut gun_query: Query<&mut Transform, (With<GlockComponent>, Without<PlayerFirstPersonCamera>)>,
    state: Res<State<CurrentWeapon>>,
) {
 
   
        if let Ok(camera_transform) = camera_query.get_single() {
            if let Ok(mut gun_transform) = gun_query.get_single_mut() {
              match state.get() {
                CurrentWeapon::Glock => {
                    let gun_rotation = camera_transform.rotation * Quat::from_rotation_y(std::f32::consts::PI);
    
                    gun_transform.rotation = gun_rotation;
        
                    let forward_vec = camera_transform.forward();
                    let camera_pitch = forward_vec.y; 
        
                    let dynamic_z_offset = if camera_pitch >= 0.0 {
                        2.9 - (camera_pitch * -5.2)  // Move closer when looking up
                    } else {
                        2.9 + (camera_pitch.abs() * -5.2)  // Move further away when looking down
                    };
        
                    // Adjust the gun's position relative to the camera
                    gun_transform.translation = camera_transform.translation
                        + camera_transform.forward() * dynamic_z_offset // Move it forward 
                        + camera_transform.right() * 0.15 // Move it to the right 
                        - camera_transform.up() * 0.1 // Move it down 
                        + Vec3::new(0.0, -4.3, 0.0); // Additional downward offset
                }

                _ => {
                    gun_transform.translation = camera_transform.translation
                    + camera_transform.forward() * 0.0 // Move it forward 
                    + camera_transform.right() * 0.15 // Move it to the right 
                    - camera_transform.up() * 10000.0 // Move it down 
                }
              }
            }
        }
    
    
   
}
