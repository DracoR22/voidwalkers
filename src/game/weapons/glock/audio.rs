use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::game::weapons::{glock::components::GlockComponent, weapon_audio::WeaponAudioList, CasingAudioTimer, WeaponAudios};

use super::resources::{GlockAudios, GlockTimer};

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
    weapon_audios_handles: Res<WeaponAudios>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>, 
    mut glock_timer: ResMut<GlockTimer>,
    mut casing_timer: ResMut<CasingAudioTimer>,
    glock_query: Query<&GlockComponent>
) {
    // reload audio
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        audio.play(weapon_audios_handles[WeaponAudioList::GLOCKRELOADFULL].clone()).handle();
    }

    // fire audio
    if let Ok(glock) = glock_query.get_single() {
      if mouse_input.just_pressed(MouseButton::Left)  {
        if glock.current_ammo > 0 {
            audio.play(weapon_audios_handles[WeaponAudioList::GLOCKFIRE].clone());
            casing_timer.timer.reset();
            casing_timer.shot_fired = true;
        } else {
            audio.play(weapon_audios_handles[WeaponAudioList::DRYFIRE].clone());
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
           audio.play(weapon_audios_handles[WeaponAudioList::GLOCKFIRE].clone()).handle();
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
         audio.play(weapon_audios_handles[WeaponAudioList::BULLETCASING].clone());
         casing_timer.shot_fired = false;  // Reset the flag
     }
}