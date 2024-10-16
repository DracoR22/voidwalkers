use bevy::prelude::*;

// ak74
#[derive(Component)]
pub struct AK74Component {
    pub current_ammo: usize,
    pub max_ammo: usize,
    pub magazine_size: usize,
    pub reload_time: f32,
    pub reload_timer: f32
}

#[derive(Component)]
pub struct HasAK74;

// glock
#[derive(Component)]
pub struct GlockComponent {
    pub current_ammo: usize,
    pub max_ammo: usize,
    pub magazine_size: usize
}

#[derive(Component)]
pub struct HasGlock;