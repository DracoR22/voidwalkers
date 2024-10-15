use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::game::weapons::{components::GlockComponent, resources::{CasingAudioTimer, GlockAudios, GlockTimer}};

pub fn setup_glock_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(GlockAudios(vec![
        asset_server.load("audios/glock-fire.ogg"),
        asset_server.load("audios/glock-reload-empty.ogg"),
        asset_server.load("audios/bullet-casing-bounce.ogg"),
    ]));
}

pub fn play_glock_audio(
    audio: Res<Audio>,
    audio_handles: Res<GlockAudios>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>, 
    mut glock_timer: ResMut<GlockTimer>,
    mut casing_timer: ResMut<CasingAudioTimer>,
    glock_query: Query<&GlockComponent>
) {
    // reload auido
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        audio.play(audio_handles.0[1].clone()).handle();
    }

    // fire audio
    if let Ok(glock) = glock_query.get_single() {
      if glock.current_ammo > 0 {
        if mouse_input.just_pressed(MouseButton::Left) {
            audio.play(audio_handles.0[0].clone());
            casing_timer.timer.reset();
            casing_timer.shot_fired = true;
        }
      }
    }

   // Update the timer
   glock_timer.0.tick(time.delta());

   // If the left mouse button is pressed
   if mouse_input.pressed(MouseButton::Left) {
       // Start the timer if it's not already running
       if glock_timer.0.finished() {
           // Play the audio
           audio.play(audio_handles.0[0].clone()).handle();
           // Reset the timer for the next interval
           glock_timer.0.reset();
            casing_timer.timer.reset();
            casing_timer.shot_fired = true;
       }
   } else {
       // reset the timer when the button is released
       glock_timer.0.reset();
   }

     // Update the casing timer
     casing_timer.timer.tick(time.delta());

     // If the casing timer has finished and a shot was fired, play the casing sound
     if casing_timer.timer.just_finished() && casing_timer.shot_fired {
         audio.play(audio_handles.0[2].clone());
         casing_timer.shot_fired = false;  // Reset the flag
     }
}