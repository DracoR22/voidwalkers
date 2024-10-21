use bevy::prelude::*;

#[derive(Component)]
pub enum EntityType {
    Cube,
    Floor,
    Wall,
    Door,
    Cat
}