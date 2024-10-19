use bevy::prelude::*;

#[derive(Component)]
pub struct FloorComponent;

#[derive(Component)]
pub struct WallComponent;

#[derive(Component)]
pub struct DoorComponent {
    pub is_opening: bool,
    pub timer: Timer,
}