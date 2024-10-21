use bevy_kira_audio::prelude::AudioSource;
use bevy::prelude::*;

#[derive(Resource)]
pub struct AK74Audios(pub Vec<Handle<AudioSource>>);

#[derive(Resource)]
pub struct AK74Timer(pub Timer);

#[derive(Resource)]
pub struct AK74Animations(pub Vec<Handle<AnimationClip>>);