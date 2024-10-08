use bevy::prelude::*;

use state::{game_state_input_events, GameState};

pub mod state;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
         .init_state::<GameState>()
         .add_systems(Update, game_state_input_events);
    }
}