use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::{common::states::CurrentWeapon, game::{player::components::{Player, PlayerFirstPersonCamera}, weapons::ak74::components::{AK74Component, HasAK74}}};

pub fn spawn_ak74(  
    mut commands: Commands,
    player_query: Query<(Entity, Option<&HasAK74>), With<Player>>, 
    ak74_query: Query<Entity, With<AK74Component>>,
    asset_server: Res<AssetServer>,
    state: Res<State<CurrentWeapon>>,
) {
   match state.get() {
    CurrentWeapon::AK74 => {
    if let Ok((player_entity, has_ak74)) = player_query.get_single() {
        if has_ak74.is_none() { 
            
            if let Ok(ak74_entity) = ak74_query.get_single() {
                println!("Reattaching AK!");
                commands.entity(player_entity).add_child(ak74_entity);

                commands.entity(player_entity).insert(HasAK74);
            }
            // } else {
            //     println!("SPAWNED AK!");
            //     commands.entity(player_entity).with_children(|parent| {
            //         parent.spawn((
            //             SceneBundle {
            //                 scene: asset_server.load("animations/ak74.glb#Scene0"),
            //                 transform: Transform {
            //                     scale: Vec3::splat(50.0), 
            //                      translation: Vec3::new(0.2, 85.5, 0.3), 
            //                      rotation: Quat::from_rotation_y(std::f32::consts::PI), 
            //                     ..default()
            //                 },
            //                 ..default()
            //             },
            //             AK74Component {
            //                 current_ammo: 35,
            //                 magazine_size: 35,
            //                 max_ammo: 325,
            //                 reload_time: 1.0,
            //                 reload_timer: 0.0
            //             }
            //         ));
            //     });
            // }

          
           
         } 
      }
    }
    _ => ()
   }
}

pub fn despawn_ak74(
    mut commands: Commands,
    mut param_set: ParamSet<(
        Query<(Entity, Option<&HasAK74>, &Transform), With<Player>>,
        Query<(Entity, &mut Transform), With<AK74Component>>,    
    )>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<CurrentWeapon>>,
    mut next_state: ResMut<NextState<CurrentWeapon>>,
) {
   match state.get() {
    CurrentWeapon::AK74 => {
     if keyboard_input.just_pressed(KeyCode::KeyQ) {
        if let Ok((player_entity, has_ak74, player_transform)) = param_set.p0().get_single() {
            if let Some(_) = has_ak74 {
                println!("DESPAWNING AK!");
                for (ak74_entity, mut ak74_transform) in param_set.p1().iter_mut() {
                    commands.entity(player_entity).remove::<HasAK74>();
                    commands.entity(player_entity).remove_children(&[ak74_entity]);

                    ak74_transform.translation = Vec3::new(0.0, 10.0, 0.0);
                 }  

                 next_state.set(CurrentWeapon::Glock);
            } 
        }
 }
    }
    _ => ()
   }
}


pub fn update_gun_rotation(
    camera_query: Query<&Transform, With<PlayerFirstPersonCamera>>,
    mut gun_query: Query<(&mut Transform, Option<&Parent>), (With<AK74Component>, Without<PlayerFirstPersonCamera>)>,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        if let Ok((mut gun_transform, maybe_parent)) = gun_query.get_single_mut() {
            if maybe_parent.is_some() {
                let gun_rotation = camera_transform.rotation * Quat::from_rotation_y(std::f32::consts::PI);

            // Update the gun's rotation
            gun_transform.rotation = gun_rotation;

            let forward_vec = camera_transform.forward();
            let camera_pitch = forward_vec.y; 

            let dynamic_z_offset = if camera_pitch >= 0.0 {
                8.9 - (camera_pitch * -5.2)  // Move closer when looking up
            } else {
                8.9 + (camera_pitch.abs() * -30.2)  // Move further away when looking down
            };

            // Adjust the gun's position relative to the camera
            gun_transform.translation = camera_transform.translation
                + camera_transform.forward() * dynamic_z_offset // Move it forward 
                + camera_transform.right() * 0.70 // Move it to the right 
                - camera_transform.up() * 1.1 // Move it down 
                + Vec3::new(0.0, -18.3, 0.0); // Additional downward offset
            }
        }
    }
}