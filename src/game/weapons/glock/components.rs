use bevy::prelude::*;

#[derive(Component)]
pub struct GlockComponent {
    pub current_ammo: usize,
    pub max_ammo: usize,
    pub magazine_size: usize,
    pub timer: f32
}

#[derive(Component)]
pub struct HasGlock;