use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
   pub speed: f32,
   pub lifetime: Timer,
}