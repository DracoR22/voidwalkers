use bevy::prelude::*;

#[derive(Debug, Clone, Eq, Default, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Playing,
    MainMenu,
    EditMode,
    Paused,
    GameOver,
}

pub fn game_state_input_events(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>
) {
    // edit mode when pressing P (todo)
 if keyboard_input.just_pressed(KeyCode::KeyP) {
    match state.get() {
        GameState::Playing => next_state.set(GameState::EditMode),
        GameState::EditMode => next_state.set(GameState::Playing),

        _ => ()
    }
 }

 // pause game when pressing ESC
 if keyboard_input.just_pressed(KeyCode::Escape) {
    match state.get() {
        GameState::Playing => next_state.set(GameState::Paused),
        GameState::Paused => next_state.set(GameState::Playing),

        _ => ()
    }
 }
// get back to game when clicking on paused game
 if mouse_input.just_pressed(MouseButton::Left) || mouse_input.just_pressed(MouseButton::Right) {
    match state.get() {
        GameState::Paused => next_state.set(GameState::Playing),
        _ => ()
    }
 }
}