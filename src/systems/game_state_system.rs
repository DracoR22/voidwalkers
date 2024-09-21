use bevy::prelude::*;

use crate::states::game_state::GameState;

pub fn game_state_input_events(mut next_state: ResMut<NextState<GameState>>, state: Res<State<GameState>>, keyboard_input: Res<ButtonInput<KeyCode>>) {
 if keyboard_input.just_pressed(KeyCode::KeyP) {
    match state.get() {
        GameState::Playing => next_state.set(GameState::EditMode),
        GameState::EditMode => next_state.set(GameState::Playing),

        _ => ()
    }
 }

 if keyboard_input.just_pressed(KeyCode::Escape) {
    match state.get() {
        GameState::Playing => next_state.set(GameState::Paused),
        GameState::Paused => next_state.set(GameState::Playing),

        _ => ()
    }
 }
}