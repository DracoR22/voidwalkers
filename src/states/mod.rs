use bevy::prelude::*;
use game_state::GameState;
use systems::game_state_input_events;

pub mod game_state;
pub mod systems;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
        .add_systems(Update, game_state_input_events);
    }
}