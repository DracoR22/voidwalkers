use bevy::prelude::*;

#[derive(Component)]
pub struct MuzzleFlash {
    pub timer: Timer,
    pub is_active: bool,
    pub frames_visible: u32
}

#[derive(Component)]
pub struct HasMuzzleFlash;

#[derive(Component)]
pub struct BloodEffectLifetime {
   pub timer: Timer,
}