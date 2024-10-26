use bevy::prelude::*;

#[derive(Debug, Clone, Eq, Default, PartialEq, Hash, States)]
pub enum CurrentWeapon {
    None,
    #[default]
    AK74,
    Glock
}