use bevy::prelude::*;
use crate::game::player::{movement::{player_look_system, player_movement_system}, shooting::{shoot_ray, update_tracers}, sight::spawn_sight_dot, spawn::spawn_player_system};

use crate::game::{state::GameState};

pub mod components;
pub mod constants;
pub mod movement;
pub mod shooting;
pub mod sight;
pub mod spawn;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
         app
        .add_systems(Startup, spawn_player_system)
        .add_systems(Startup, spawn_sight_dot)
        .add_systems(Update, shoot_ray)
        .add_systems(Update, (
            player_movement_system,
            player_look_system,
            update_tracers,
        )
        .chain()
        .run_if(in_state(GameState::Playing)),
    );
    }
}