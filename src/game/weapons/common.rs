use bevy::prelude::*;

use super::{components::{AK74Component, GlockComponent}, glock::GLOCK_MAGAZINE_SIZE, state::CurrentWeapon};

#[derive(Resource)]
pub struct WeaponFireTimer(pub Timer);

pub fn reload_weapon(
    weapon_state: Res<State<CurrentWeapon>>,
    mut glock_query: Query<&mut GlockComponent>,
    mut ak74_query: Query<&mut AK74Component>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {
  match weapon_state.get() {
    CurrentWeapon::Glock => {
       if let Ok(mut glock) = glock_query.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::KeyR) && glock.current_ammo != GLOCK_MAGAZINE_SIZE {
            glock.current_ammo += GLOCK_MAGAZINE_SIZE - glock.current_ammo;
        }
       }
    },

    CurrentWeapon::AK74 => {
        
    }

    CurrentWeapon::None => {}
  }
}

pub fn can_shoot_and_decrease_ammo(
    weapon_state: &CurrentWeapon,
    glock_query: &mut Query<&mut GlockComponent>,
    ak74_query: &mut Query<&mut AK74Component>,
) -> bool {
    match weapon_state {
        CurrentWeapon::Glock => {
            if let Ok(mut glock) = glock_query.get_single_mut() {
                if glock.current_ammo > 0 {
                    glock.current_ammo -= 1;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        },
        CurrentWeapon::AK74 => {
            if let Ok(mut ak74) = ak74_query.get_single_mut() {
                if ak74.current_ammo > 0 {
                    ak74.current_ammo -= 1;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        },
        CurrentWeapon::None => false
    }
}

pub fn update_weapon_timer(
    weapon_state: Res<State<CurrentWeapon>>, // Access current weapon
    mut fire_timer: ResMut<WeaponFireTimer>, // Access the timer to modify
) {
    let mut duration_changed = false;

    match weapon_state.get() {
        CurrentWeapon::Glock => {
            if fire_timer.0.duration() != std::time::Duration::from_secs_f32(0.5) {
                fire_timer.0.set_duration(std::time::Duration::from_secs_f32(0.5)); // Glock fire rate
                duration_changed = true;
            }
        }
        CurrentWeapon::AK74 => {
            if fire_timer.0.duration() != std::time::Duration::from_secs_f32(0.1) {
                fire_timer.0.set_duration(std::time::Duration::from_secs_f32(0.1)); // AK-74 fire rate
                duration_changed = true;
            }
        }
        CurrentWeapon::None => {
            // You might not want to set a timer for "None", but for completeness:
            if fire_timer.0.duration() != std::time::Duration::from_secs_f32(0.0) {
                fire_timer.0.set_duration(std::time::Duration::from_secs_f32(0.0));
                duration_changed = true;
            }
        }
    }

    // Only reset the timer if the duration changed
    if duration_changed {
        fire_timer.0.reset();
    }
}
