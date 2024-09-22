use bevy::prelude::*;

#[derive(Resource)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);