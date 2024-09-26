use bevy::prelude::*;

#[derive(Resource)]
pub struct CatAnimations(pub Vec<Handle<AnimationClip>>);