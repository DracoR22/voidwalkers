use bevy::prelude::*;

use crate::{states::game_state::GameState, systems::game_state_system::game_state_input_events};

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
        .add_systems(Update, game_state_input_events);
    }
}