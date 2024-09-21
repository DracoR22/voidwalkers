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