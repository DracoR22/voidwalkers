use bevy::prelude::*;

use bevy_kira_audio::prelude::*;
use bevy_kira_audio::AudioSource;

use std::ops::Index;

use crate::common::commands::action_from_input;
use crate::common::commands::Action;
use crate::game::player::audio::PlayerAudioList::Step1;

#[derive(Resource)]
pub struct StepTimer(pub Timer);

#[derive(Resource, Default)]
pub struct PlayerAudios(pub Vec<Handle<AudioSource>>);

pub fn setup_player_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(PlayerAudios(vec![
        asset_server.load("audios/player_step_1.ogg"),
        asset_server.load("audios/player_step_2.ogg"),
        asset_server.load("audios/player_step_3.ogg"),
        asset_server.load("audios/player_step_4.ogg"),
    ]));
}

#[derive(PartialEq, Clone, Copy)]
pub enum PlayerAudioList {
    Step1,
    Step2,
    Step3,
    Step4
}
impl Default for PlayerAudioList {
    fn default() -> Self {
        self::Step1
    }
}

impl Index<PlayerAudioList> for PlayerAudios {
    type Output = Handle<AudioSource>;

    fn index(&self, index: PlayerAudioList) -> &Self::Output {
        &self.0[index as usize]
    }
}

pub fn play_player_audio(
    audio: Res<Audio>,
    audio_handles: Res<PlayerAudios>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut step_timer: ResMut<StepTimer>,
    mut last_step: Local<usize>,
    time: Res<Time>
) {
    let actions = action_from_input(&keyboard_input);

    let is_running = actions.contains(&Action::Run);

    if is_running {
        step_timer.0.set_duration(std::time::Duration::from_secs_f32(0.25)); // Faster interval for running
    } else {
        step_timer.0.set_duration(std::time::Duration::from_secs_f32(0.5)); // Default interval for walking
    }

  step_timer.0.tick(time.delta());
  
  if step_timer.0.finished() {

  for action in actions {
    match action {
        Action::WalkForward | Action::WalkBackward | Action::WalkRightward | Action::WalkLeftward | Action::Run => {
            // Play the next footstep sound in sequence
            let step_sound = match *last_step % 4 {
                0 => &audio_handles[PlayerAudioList::Step1],
                1 => &audio_handles[PlayerAudioList::Step2],
                2 => &audio_handles[PlayerAudioList::Step3],
                _ => &audio_handles[PlayerAudioList::Step4],
            };
            
            audio.play(step_sound.clone());
            *last_step += 1; // Advance to the next sound for variety

            break; // Only play one sound per step cycle
        }
        _ => (),
    }
  }
  }
}