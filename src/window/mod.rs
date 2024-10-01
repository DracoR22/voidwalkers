use bevy::prelude::*;
use cursor::{handle_pause_state, setup_window, toggle_cursor};

pub mod cursor;

pub struct WindowSetupPlugin;

impl Plugin for WindowSetupPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_window)
        .add_systems(Update,  (
            toggle_cursor,
            handle_pause_state
        ));
    }
}