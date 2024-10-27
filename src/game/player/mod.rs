use audio::{play_player_audio, setup_player_audio, StepTimer};
use bevy::prelude::*;
use spawn::{disable_player_physics_system, enable_player_physics_system};
use crate::game::player::{movement::{player_look_system, player_movement_system, player_movement_editor_system}, shooting::{shoot_ray, update_tracers}, sight::spawn_sight_dot, spawn::spawn_player_system};

use crate::game::{state::GameState};

pub mod components;
pub mod constants;
pub mod movement;
pub mod shooting;
pub mod sight;
pub mod spawn;
pub mod audio;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
         app
         .insert_resource(StepTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .add_systems(Startup, (
            spawn_player_system,
            setup_player_audio
        ))
        .add_systems(Startup, spawn_sight_dot)
        .add_systems(Update, shoot_ray)
        .add_systems(Update, (
            player_movement_editor_system, 
            player_look_system, 
            enable_player_physics_system
        ).chain()
        .run_if(in_state(GameState::EditMode)))
        .add_systems(Update, (
            player_movement_system,
            player_look_system, 
            update_tracers,
            play_player_audio,
            disable_player_physics_system
        )
        .chain()
        .run_if(in_state(GameState::Playing)),
    );
    }
}