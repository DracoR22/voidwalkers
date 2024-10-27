use bevy::prelude::*;

#[derive(Component)]
pub enum EntityType {
    Cube,
    Floor,
    Wall,
    Door,
    Cat
}

#[derive(Component, PartialEq)]
pub enum WeaponType {
    Ak74,
    Glock
}