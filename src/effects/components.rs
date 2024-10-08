use bevy::prelude::*;

#[derive(Component)]
pub struct MuzzleFlash {
    pub timer: Timer,
}

#[derive(Component)]
pub struct HasMuzzleFlash;

#[derive(Component)]
pub struct BloodEffectLifetime {
   pub timer: Timer,
}