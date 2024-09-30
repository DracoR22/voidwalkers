use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::weapons::resources::{AK74Audios, AK74Timer};

pub fn setup_ak74_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
 commands.insert_resource(AK74Audios(vec![
    asset_server.load("audios/ak74.ogg"),
    asset_server.load("audios/mag-reload-rifle.ogg")
 ]));
}

pub fn play_ak74_reload(audio: Res<Audio>, audio_handles: Res<AK74Audios>, keyboard_input: Res<ButtonInput<KeyCode>>, mut current_audio: Local<usize>, mut players_query: Query<&mut AnimationPlayer>) {
  if keyboard_input.just_pressed(KeyCode::KeyR) {
    audio.play(audio_handles.0[1].clone()).handle();
  }
}

pub fn play_ak74_audio(audio: Res<Audio>, audio_handles: Res<AK74Audios>, mouse_input: Res<ButtonInput<MouseButton>>, time: Res<Time>, mut ak74_timer: ResMut<AK74Timer>,) {

    if mouse_input.just_pressed(MouseButton::Left) {
        audio.play(audio_handles.0[0].clone()).handle();
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
       }
   } else {
       // Optionally, reset the timer when the button is released
       ak74_timer.0.reset();
   }
}