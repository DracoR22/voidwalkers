use bevy::prelude::*;

use crate::{resources::projectile_properties::ProjectileProperties, states::game_state::GameState, systems::{player_system::{player_look_system, player_movement_system, spawn_player_system}, shooting_system::{projectile_collision_system, setup_shooting, shooting_pr_system, update_projectiles}}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<ProjectileProperties>() // Initialize the resource
            .add_systems(Startup, setup_shooting)
        .add_systems(Startup, spawn_player_system)
        .add_systems(Update, (
            player_movement_system,
            player_look_system,
            shooting_pr_system,
                    update_projectiles,
                    projectile_collision_system,
        )
        .chain()
        .run_if(in_state(GameState::Playing)),
    );
    }
}