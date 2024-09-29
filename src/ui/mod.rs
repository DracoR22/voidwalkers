use bevy::prelude::*;
use position::{update_player_position, write_player_position};

pub mod position;

pub struct GameUIPugin;

impl Plugin for GameUIPugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, write_player_position)
        .add_systems(Update, update_player_position);
    }
}