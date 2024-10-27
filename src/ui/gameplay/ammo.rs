use bevy::prelude::*;

use crate::{common::states::CurrentWeapon, game::weapons::{ak74::components::AK74Component, glock::components::GlockComponent}};

#[derive(Component)]
pub struct AmmoText;

pub fn write_gun_ammo(mut commands: Commands, asset_server: Res<AssetServer>) {
 let font_handle = asset_server.load("fonts/KONSTANTINE.ttf");

 let ammo_style = TextStyle {
    font: font_handle.clone(),
    font_size: 24.0,
    color: Color::GREEN, // First number in green
};

let current_ammo_style = TextStyle {
    font: font_handle.clone(),
    font_size: 24.0,
    color: Color::WHITE, // Second number in white
};

 commands.spawn((
    AmmoText,
    TextBundle::from_sections(vec![
        TextSection::new("0", ammo_style.clone()), // First number
        TextSection::new("/0", current_ammo_style.clone()), // Second number
    ])
    .with_style(Style {
        position_type: PositionType::Absolute,
        bottom: Val::Px(35.0),
        right: Val::Px(40.0),
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
                    ammo_text.sections[0].value = format!("{}", glock.current_ammo); 
                    ammo_text.sections[1].value = format!("/{}", glock.max_ammo)
                }
            },
            CurrentWeapon::AK74 => {
                if let Ok(ak74) = ak74_query.get_single() {
                    ammo_text.sections[0].value = format!("{}", ak74.current_ammo); 
                    ammo_text.sections[1].value = format!("/{}", ak74.max_ammo)
                }
            },
            CurrentWeapon::None => {
                ammo_text.sections[0].value = "0".to_string(); 
                ammo_text.sections[1].value = "/0".to_string();
            }
        }
    }
}