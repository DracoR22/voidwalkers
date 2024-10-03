use bevy::prelude::*;

#[derive(Debug, Clone, Eq, Default, PartialEq, Hash, States)]
pub enum CurrentWeapon {
    None,
    AK74,
    #[default]
    Glock
}
