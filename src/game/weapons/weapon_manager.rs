use bevy::prelude::*;

use crate::{common::{entities::WeaponType, states::CurrentWeapon}, game::player::components::Player};

use super::HasWeapon;

pub fn equip_weapon(
    mut commands: Commands,
    player_query: Query<(Entity, Option<&HasWeapon>), With<Player>>, 
    weapon_query: Query<(Entity, &WeaponType)>,
    state: Res<State<CurrentWeapon>>,
) {
    if let Ok((player_entity, has_weapon)) = player_query.get_single() {
        match state.get() {
            CurrentWeapon::AK74 => {
                if has_weapon.map(|hw| hw.0 != WeaponType::Ak74).unwrap_or(true) { 

                    if let Some((ak74_entity, _)) = weapon_query.iter().find(|(_, weapon_type)| **weapon_type == WeaponType::Ak74) {
                        println!("Reattaching AK!");
                        commands.entity(player_entity).add_child(ak74_entity);
        
                        // Mark that the player now has the AK74
                        commands.entity(player_entity).insert(HasWeapon(WeaponType::Ak74));
                    }
                }
            }

            CurrentWeapon::Glock => {
                if has_weapon.map(|hw| hw.0 != WeaponType::Glock).unwrap_or(true)  {
                    // If there is no Glock yet, we spawn a new one
                    if let Some((glock_entity, _)) = weapon_query.iter().find(|(_, weapon_type)| **weapon_type == WeaponType::Glock) {
                        println!("Reattaching AK!");
                        commands.entity(player_entity).add_child(glock_entity);
        
                        // Mark that the player now has the AK74

                        commands.entity(player_entity).insert(HasWeapon(WeaponType::Glock));
                    }
                }
            }
              _ => ()
          }
    }
}

pub fn unequip_weapon(
    mut commands: Commands,
    mut param_set: ParamSet<(
        Query<(Entity, Option<&HasWeapon>, &Transform), With<Player>>,
        Query<(Entity, &mut Transform), With<WeaponType>>,    
    )>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<CurrentWeapon>>,
    mut next_state: ResMut<NextState<CurrentWeapon>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        if let Ok((player_entity, has_weapon, player_transform)) = param_set.p0().get_single() {
          match state.get() {
            CurrentWeapon::AK74 => {
                if has_weapon.map(|hw| hw.0 == WeaponType::Ak74).unwrap_or(true) {
                    println!("DESPAWNING AK!");
                    for (ak74_entity, mut ak74_transform) in param_set.p1().iter_mut() {
                        commands.entity(player_entity).remove::<HasWeapon>();
                        commands.entity(player_entity).remove_children(&[ak74_entity]);
    
                        ak74_transform.translation = Vec3::new(0.0, 10.0, 0.0);
                     }  
    
                     next_state.set(CurrentWeapon::Glock);
                } 
            }

            CurrentWeapon::Glock => {
                if has_weapon.map(|hw| hw.0 == WeaponType::Glock).unwrap_or(true) {
                    for (glock_entity, mut glock_transform) in param_set.p1().iter_mut() {
                        commands.entity(player_entity).remove::<HasWeapon>();
                        commands.entity(glock_entity).remove_parent();
    
                        glock_transform.translation = Vec3::new(0.0, 10.0, 0.0);
                     }  
    
                     next_state.set(CurrentWeapon::AK74);
                  }
            }

            _ => ()
          }
        }
    }
}