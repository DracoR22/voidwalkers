use bevy::prelude::*;

use player::PlayerPlugin;
use state::{game_state_input_events, GameState};
use weapons::{ WeaponsPlugin};

use crate::common::states::CurrentWeapon;

pub mod state;
pub mod weapons;
pub mod player;
pub mod map;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
         .init_state::<GameState>()
         .init_state::<CurrentWeapon>()
         .add_plugins(PlayerPlugin)
         .add_plugins(WeaponsPlugin)
         .add_systems(Update, game_state_input_events);
    }
}