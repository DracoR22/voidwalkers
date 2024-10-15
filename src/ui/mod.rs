use bevy::prelude::*;
use fps::{update_game_fps, write_game_fps, GameFPSText, GameFpsRoot};
use gameplay::GamePlayUIPlugin;
use position::{update_player_position, write_player_position, PlayerPosition};

pub mod position;
pub mod fps;
pub mod gameplay;

#[derive(Component)]
pub struct DebugMenu;

pub struct GameUIPugin;

impl Plugin for GameUIPugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(GamePlayUIPlugin)
        .add_systems(Startup, write_debug_menu)
        .add_systems(Update, (
            update_player_position, 
            update_game_fps,
            showhide_debug_menu
        ));
    }
}

pub fn write_debug_menu(mut commands: Commands) {
   // Spawn the parent entity with the DebugMenu component
     let debug_menu_entity = commands.spawn((
        DebugMenu,
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column, // Stack items vertically
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                padding: UiRect::all(Val::Px(4.0)),
                ..Default::default()
            },
            
            ..Default::default()
        },
    )).id();

    let text_fps = write_game_fps(&mut commands);
    let text_position = write_player_position(&mut commands);
    
    commands.entity(debug_menu_entity).push_children(&[text_fps]);
    commands.entity(debug_menu_entity).push_children(&[text_position]);
}

pub fn showhide_debug_menu(
    mut q: Query<&mut Visibility, With<DebugMenu>>, // Query for DebugMenu component
    kbd: Res<ButtonInput<KeyCode>>,  // Keyboard input resource
) {
    if kbd.just_pressed(KeyCode::F12) {
        // Access the Visibility component for the DebugMenu
        let mut vis = q.single_mut();
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}