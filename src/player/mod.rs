use bevy::prelude::*;
use systems::{animations::load_animation, movement::{player_look_system, player_movement_system, update_gun_rotation}, shooting::{shoot_ray, update_tracers}, sight::spawn_sight_dot, spawn::spawn_player_system};

use crate::states::game_state::GameState;

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
        .add_systems(Update,  load_animation)

        .add_systems(Update, (
            player_movement_system,
            player_look_system,
            shoot_ray,
            update_tracers,
            update_gun_rotation
        )
        .chain()
        .run_if(in_state(GameState::Playing)),
    );
    }
}