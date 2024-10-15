use bevy::prelude::*;

use state::{game_state_input_events, GameState};
use weapons::{states::CurrentWeapon, WeaponsPlugin};

pub mod state;
pub mod weapons;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
         .init_state::<GameState>()
         .init_state::<CurrentWeapon>()
         .add_plugins(WeaponsPlugin)
         .add_systems(Update, game_state_input_events);
    }
}