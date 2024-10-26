use bevy::prelude::*;

use crate::common::states::CurrentWeapon;

use super::{
    ak74::components::AK74Component,
    glock::{components::GlockComponent, Weapon, GLOCK_MAGAZINE_SIZE, MAX_GLOCK_AMMO},
};

#[derive(Resource)]
pub struct WeaponFireTimer(pub Timer);

pub fn spawn_weapons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("animations/ak74.glb#Scene0"),
            transform: Transform {
                scale: Vec3::splat(50.0),
                translation: Vec3::new(0.2, 85.5, 0.3),
                rotation: Quat::from_rotation_y(std::f32::consts::PI),
                ..default()
            },
            ..default()
        },
        AK74Component {
            current_ammo: 35,
            magazine_size: 35,
            max_ammo: 325,
            reload_time: 1.0,
            reload_timer: 0.0,
        },
    ));

    commands.spawn((
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
}

pub fn reload_weapon(
    weapon_state: Res<State<CurrentWeapon>>,
    mut glock_query: Query<&mut GlockComponent>,
    mut ak74_query: Query<&mut AK74Component>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    match weapon_state.get() {
        CurrentWeapon::Glock => {
            if let Ok(mut glock) = glock_query.get_single_mut() {
                if keyboard_input.just_pressed(KeyCode::KeyR)
                    && glock.current_ammo != GLOCK_MAGAZINE_SIZE
                {
                    glock.current_ammo += GLOCK_MAGAZINE_SIZE - glock.current_ammo;
                }
            }
        }

        CurrentWeapon::AK74 => {}

        CurrentWeapon::None => {}
    }
}

pub fn can_shoot_and_decrease_ammo<T: Weapon + Component>(mut weapon_query: Query<&mut T>) -> bool {
    if let Ok(mut weapon) = weapon_query.get_single_mut() {
        if weapon.current_ammo() > 0 {
            weapon.decrease_ammo();
            return true;
        }
    }
    false
}

pub fn update_weapon_timer(
    weapon_state: Res<State<CurrentWeapon>>, // Access current weapon
    mut fire_timer: ResMut<WeaponFireTimer>, // Access the timer to modify
) {
    let mut duration_changed = false;

    match weapon_state.get() {
        CurrentWeapon::Glock => {
            if fire_timer.0.duration() != std::time::Duration::from_secs_f32(0.1) {
                fire_timer
                    .0
                    .set_duration(std::time::Duration::from_secs_f32(0.1)); // Glock fire rate
                duration_changed = true;
            }
        }
        CurrentWeapon::AK74 => {
            if fire_timer.0.duration() != std::time::Duration::from_secs_f32(0.1) {
                fire_timer
                    .0
                    .set_duration(std::time::Duration::from_secs_f32(0.1)); // AK-74 fire rate
                duration_changed = true;
            }
        }
        CurrentWeapon::None => {
            // You might not want to set a timer for "None", but for completeness:
            if fire_timer.0.duration() != std::time::Duration::from_secs_f32(0.0) {
                fire_timer
                    .0
                    .set_duration(std::time::Duration::from_secs_f32(0.0));
                duration_changed = true;
            }
        }
    }

    // Only reset the timer if the duration changed
    if duration_changed {
        fire_timer.0.reset();
    }
}
