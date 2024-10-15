use bevy::prelude::*;

use super::components::{AK74Component, GlockComponent};

#[derive(Debug, Clone, Eq, Default, PartialEq, Hash, States)]
pub enum CurrentWeapon {
    None,
    AK74,
    #[default]
    Glock
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