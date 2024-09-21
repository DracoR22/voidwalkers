use bevy::prelude::*;

use crate::{states::game_state::GameState, systems::{player_system::{player_look_system, player_movement_system, spawn_player_system}, shooting_system::{shoot_ray, update_tracers}}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player_system)
        .add_systems(Update, (
            player_movement_system,
            player_look_system,
            shoot_ray,
            update_tracers
        )
        .chain()
        .run_if(in_state(GameState::Playing)),
    );
    }
}