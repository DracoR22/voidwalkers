use bevy::prelude::*;

#[derive(Component)]
pub enum CurrentWeapon {
    None,
    AK74,
    Glock,
}

#[derive(Debug, Clone, Eq, Default, PartialEq, Hash, States)]
pub enum CurrentWeaponState {
    None,
    AK74,
    #[default]
    Glock
}

#[derive(Component)]
pub struct AK74Component;

#[derive(Component)]
pub struct HasAK74;

#[derive(Component)]
pub struct GlockComponent;

#[derive(Component)]
pub struct HasGlock;