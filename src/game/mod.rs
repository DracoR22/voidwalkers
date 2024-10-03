use bevy::prelude::*;
use blood::{cleanup_blood_effects, spawn_blood_mesh};
use state::{game_state_input_events, GameState};

pub mod blood;
pub mod link_animations;
pub mod state;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
         .init_state::<GameState>()
         .add_systems(Startup,  spawn_blood_mesh)
         .add_systems(Update, (
            game_state_input_events,
            cleanup_blood_effects
         ));
    }
}