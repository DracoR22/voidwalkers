use bevy::prelude::*;
use systems::{movement::{player_look_system, player_movement_system}, shooting::{shoot_ray, update_tracers}, sight::spawn_sight_dot, spawn::spawn_player_system};

use crate::game::{link_animations::link_animations, state::GameState};

pub mod components;
pub mod resources;
pub mod systems;
pub mod constants;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
         app
        //  .insert_resource(Animations(Vec::new()))
        .add_systems(Startup, spawn_player_system)
        .add_systems(Startup, spawn_sight_dot)
       

        .add_systems(Update, (
            player_movement_system,
            player_look_system,
            shoot_ray,
            update_tracers,
        )
        .chain()
        .run_if(in_state(GameState::Playing)),
    );
    }
}