use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationState {
   pub current_animation: usize,
}

#[derive(Component)]
pub struct CatComponent;