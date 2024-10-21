use bevy_kira_audio::prelude::AudioSource;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GlockAnimations(pub Vec<Handle<AnimationClip>>);

#[derive(Resource)]
pub struct GlockAudios(pub Vec<Handle<AudioSource>>);

#[derive(Resource)]
pub struct GlockTimer(pub Timer);