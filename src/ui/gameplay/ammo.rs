use bevy::prelude::*;

use crate::game::weapons::{components::{AK74Component, GlockComponent}, states::CurrentWeapon};

#[derive(Component)]
pub struct AmmoText;

pub fn write_gun_ammo(mut commands: Commands) {
 let style = TextStyle::default();

 commands.spawn((
    AmmoText,
    TextBundle::from_sections(vec![TextSection::new(
        "0 / 0",
        style.clone()
    )])
    .with_style(Style {
        position_type: PositionType::Absolute,
        bottom: Val::Px(12.0),
        right: Val::Px(12.0),
        ..default()
    })
 ));
}

pub fn update_gun_ammo(
    glock_query: Query<&GlockComponent>,
    mut text_query: Query<&mut Text, With<AmmoText>>,
    weapon_state: Res<State<CurrentWeapon>>,
    ak74_query: Query<&AK74Component>,
) {
    if let Ok(mut ammo_text) = text_query.get_single_mut() {
        match weapon_state.get() {
            CurrentWeapon::Glock => {
                if let Ok(glock) = glock_query.get_single() {
                    ammo_text.sections[0].value = format!("{} / {}", glock.current_ammo, glock.max_ammo);
                }
            },
            CurrentWeapon::AK74 => {
                if let Ok(ak74) = ak74_query.get_single() {
                    ammo_text.sections[0].value = format!("{} / {}", ak74.current_ammo, ak74.max_ammo);
                }
            },
            CurrentWeapon::None => {
                ammo_text.sections[0].value = "0 / 0".to_string();
            }
        }
    }
}