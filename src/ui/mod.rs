use bevy::prelude::*;
use fps::{update_game_fps, write_game_fps};
use position::{update_player_position, write_player_position};

pub mod position;
pub mod fps;

pub struct GameUIPugin;

impl Plugin for GameUIPugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, (
            write_player_position,
            write_game_fps
        ))
        .add_systems(Update, (
            update_player_position, 
            update_game_fps
        ));
    }
}