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
    pub magazine_size: usize,
    pub timer: f32
}

#[derive(Component)]
pub struct HasGlock;

// traits
pub trait Weapon {
    fn current_ammo(&self) -> usize;
    fn decrease_ammo(&mut self);
}

impl Weapon for GlockComponent {
    fn current_ammo(&self) -> usize {
        self.current_ammo
    }

    fn decrease_ammo(&mut self) {
        if self.current_ammo > 0 {
            self.current_ammo -= 1;
        }
    }
}

impl Weapon for AK74Component {
    fn current_ammo(&self) -> usize {
        self.current_ammo
    }

    fn decrease_ammo(&mut self) {
        if self.current_ammo > 0 {
            self.current_ammo -= 1;
        }
    }
}