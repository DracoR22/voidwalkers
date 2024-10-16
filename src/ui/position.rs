use bevy::prelude::*;

use crate::game::player::components::Player;

#[derive(Component)]
pub struct PlayerPosition;

pub fn write_player_position(commands: &mut Commands) -> Entity {
  let style = TextStyle::default();

  let text_position = commands.spawn((
    PlayerPosition,
    TextBundle::from_sections(vec![TextSection::new(
        "Pos: x: 0.0, y: 0.0, z: 0.0",
        style.clone(),
    )])
    .with_style(Style {
        // position_type: PositionType::Absolute,
        // top: Val::Px(12.0),
        // left: Val::Px(12.0),
        ..default()
    }),
  )).id();

  return text_position
}

pub fn update_player_position(
  mut commands: Commands,
  player_query: Query<&Transform, With<Player>>,
  mut text_query: Query<&mut Text, With<PlayerPosition>>,
) {
  if let Ok(player_transform) = player_query.get_single() {
    let player_position = player_transform.translation;

    if let Ok(mut text) = text_query.get_single_mut() {
        // Update the text with the player's position
        text.sections[0].value = format!(
            "Pos: x: {:.2}, y: {:.2}, z: {:.2}",
            player_position.x, player_position.y, player_position.z
        );
    }
}
}