use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::weapons::resources::{AK74Audios, AK74Timer, CasingAudioTimer};

pub fn setup_ak74_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
 commands.insert_resource(AK74Audios(vec![
    asset_server.load("audios/ak74-fire.ogg"),
    asset_server.load("audios/ak74-reload-empty.ogg"),
    asset_server.load("audios/bullet-casing-bounce.ogg"),
 ]));
}

pub fn play_ak74_audio(
    audio: Res<Audio>,
    audio_handles: Res<AK74Audios>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut ak74_timer: ResMut<AK74Timer>,
    mut casing_timer: ResMut<CasingAudioTimer>,
) {
    // reload audio
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        audio.play(audio_handles.0[1].clone()).handle();
    }

    // fire audio
    if mouse_input.just_pressed(MouseButton::Left) {
        audio.play(audio_handles.0[0].clone());
        casing_timer.timer.reset();
        casing_timer.shot_fired = true;
    }

   // Update the timer
   ak74_timer.0.tick(time.delta());

   // If the left mouse button is pressed
   if mouse_input.pressed(MouseButton::Left) {
       // Start the timer if it's not already running
       if ak74_timer.0.finished() {
           // Play the audio
           audio.play(audio_handles.0[0].clone()).handle();
           // Reset the timer for the next interval
           ak74_timer.0.reset();
           casing_timer.timer.reset();
           casing_timer.shot_fired = true;
       }
   } else {
       // Optionally, reset the timer when the button is released
       ak74_timer.0.reset();
   }

   // Update the casing timer
   casing_timer.timer.tick(time.delta());

   // If the casing timer has finished and a shot was fired, play the casing sound
   if casing_timer.timer.just_finished() && casing_timer.shot_fired {
       audio.play(audio_handles.0[2].clone());
       casing_timer.shot_fired = false;  // Reset the flag
   }
}