use bevy::prelude::*;

#[derive(Debug, Clone, Eq, Default, PartialEq, Hash, States)]
pub enum BloodState {
    #[default]
    None,
    Spawned,
}