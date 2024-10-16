use bevy::prelude::*;

use crate::game::player::components::SightDot;

pub fn spawn_sight_dot(mut commands: Commands, assets_server: Res<AssetServer>) {
commands.spawn((
    NodeBundle {
        style: Style {
            width: Val::Px(6.0),
            height: Val::Px(6.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            
            ..default()
        },
        ..default()
    },
    SightDot
)).with_children(|parent| {
    parent.spawn(ImageBundle {
        image: UiImage {
            texture: assets_server.load("images/crosshair.png"),
            ..default()
        },
        ..default()
    });
});
}