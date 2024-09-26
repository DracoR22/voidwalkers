use bevy::prelude::*;
use cat::{animations::play_cat_animation, spawn::spawn_cat};

use crate::game::link_animations::link_animations;

pub mod cat;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_cat)
        .add_systems(Update, (
            play_cat_animation,
            link_animations
        ));
    }
}