use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerFirstPersonCamera;

#[derive(Component)]
pub struct BulletTracer {
    pub start_position: Vec3,
    pub end_position: Vec3,
    pub life_time: f32,
    pub direction: Vec3,
}