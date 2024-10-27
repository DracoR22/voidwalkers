use bevy::prelude::*;

#[derive(PartialEq)]
pub enum Action {
    WalkForward,
    WalkBackward,
    WalkLeftward,
    WalkRightward,
    Run,
    Jump,
    Reload,
    Shoot,
    Crouch,
    TogglePhysics,
    Interact,
    ToggleDebugMenu
}

pub fn action_from_input(keyboard_input: &Res<ButtonInput<KeyCode>>) -> Vec<Action> {
    let mut actions = Vec::new();

    if keyboard_input.pressed(KeyCode::KeyW) {
        actions.push(Action::WalkForward);
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        actions.push(Action::WalkBackward);
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        actions.push(Action::WalkLeftward);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        actions.push(Action::WalkRightward);
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        actions.push(Action::Run);
    }
    if keyboard_input.pressed(KeyCode::Space) {
        actions.push(Action::Jump);
    }
    if keyboard_input.pressed(KeyCode::ControlLeft) {
        actions.push(Action::Crouch);
    }


    // Use just_pressed for actions that should only happen once per press
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        actions.push(Action::TogglePhysics);
    }
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        actions.push(Action::Interact);
    }

    actions
}
