use bevy_kira_audio::prelude::AudioSource;
use bevy::prelude::*;

#[derive(Resource)]
pub struct CasingAudioTimer {
    pub timer: Timer,
    pub shot_fired: bool,
}

#[derive(Resource, Default)]
pub struct WeaponAudios(pub Vec<Handle<AudioSource>>);

// ak74
#[derive(Resource)]
pub struct AK74Audios(pub Vec<Handle<AudioSource>>);

#[derive(Resource)]
pub struct AK74Timer(pub Timer);

#[derive(Resource)]
pub struct AK74Animations(pub Vec<Handle<AnimationClip>>);

// glock
#[derive(Resource)]
pub struct GlockAnimations(pub Vec<Handle<AnimationClip>>);

#[derive(Resource)]
pub struct GlockAudios(pub Vec<Handle<AudioSource>>);

#[derive(Resource)]
pub struct GlockTimer(pub Timer);