use bevy::prelude::*;

use super::components::{AK74Component, GlockComponent};

#[derive(Debug, Clone, Eq, Default, PartialEq, Hash, States)]
pub enum CurrentWeapon {
    None,
    AK74,
    #[default]
    Glock
}