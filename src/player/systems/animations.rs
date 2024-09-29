use std::time::Duration;
use bevy::{animation, prelude::*};

use crate::{game::link_animations::AnimationEntityLink, player::{components::Player, resources::Animations}};

pub fn load_animation(
    animations: Res<Animations>,
    mut players_query: Query<&mut AnimationPlayer>,
    mut current_animation: Local<usize>,
    mut player_character_query: Query<(&Player, &AnimationEntityLink)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {

    for (player_entity, animation_entity_link) in &mut player_character_query.iter_mut() {
        if let Ok(mut animation_player) = players_query.get_mut(animation_entity_link.0) {
            keyboard_input.get_just_pressed().into_iter().for_each(|key_code| {

                match key_code {
                    KeyCode::KeyQ => *current_animation = 1,
                    KeyCode::KeyW => *current_animation = 2,
                    KeyCode::KeyR => *current_animation = 7,
                    // do the same for all animations
        
                    _ => ()
                }
            });
    
            if mouse_input.pressed(MouseButton::Left) {
                // Continuously set the animation to 4 while the mouse button is held
                *current_animation = 4;
            }
        
            if mouse_input.just_released(MouseButton::Left) {
                // Stop looping or switch animation when left-click is released
                *current_animation = 0; // Reset or change animation on release
            }
    
            if keyboard_input.just_released(KeyCode::KeyW) {
                *current_animation = 0;
            }
    
            let animation: &mut AnimationPlayer = animation_player.play_with_transition(
                animations.0[*current_animation].clone_weak(), 
                Duration::from_millis(100), // transition duration
            );

            
    
                // player.play(animations.0[*current_animation].clone_weak());
    
                if *current_animation == 2 {
                    animation.repeat();
                }
    
                if *current_animation == 4 {
                    animation.repeat();
                    animation.set_speed(1.0); 
                }
        
        }
    }
}