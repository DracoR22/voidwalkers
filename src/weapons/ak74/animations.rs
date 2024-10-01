use bevy::prelude::*;
use std::time::Duration;
use crate::{game::link_animations::AnimationEntityLink, player::components::Player, weapons::resources::AK74Animations};

#[derive(PartialEq, Clone, Copy)]
pub enum AK74AnimationsList {
  IDLE,
  DRAW,
  WALK,
  RUN,
  SHOOT,
  RELOAD,
  RELOADFAST,
  RELOADFULL
}

impl Default for AK74AnimationsList {
    fn default() -> Self {
        Self::IDLE
    }
}

impl From<&KeyCode> for AK74AnimationsList {
    fn from(key_code: &KeyCode) -> Self {
        match key_code {
            KeyCode::KeyQ => AK74AnimationsList::DRAW,
            KeyCode::KeyW => AK74AnimationsList::WALK,
            KeyCode::KeyA => AK74AnimationsList::WALK,
            KeyCode::KeyS => AK74AnimationsList::WALK,
            KeyCode::KeyD => AK74AnimationsList::WALK,
            KeyCode::KeyR => AK74AnimationsList::RELOADFULL,
            // do the same for all animations

            _ => AK74AnimationsList::IDLE,
        }
    }
}

pub fn setup_ak74_animations(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AK74Animations(vec![
        asset_server.load("animations/ak.glb#Animation0"), // add more animations
        asset_server.load("animations/ak.glb#Animation1"),
        asset_server.load("animations/ak.glb#Animation2"),
        asset_server.load("animations/ak.glb#Animation3"),
        asset_server.load("animations/ak.glb#Animation4"),
        asset_server.load("animations/ak.glb#Animation5"),
        asset_server.load("animations/ak.glb#Animation6"),
        asset_server.load("animations/ak.glb#Animation7")
    ]));
}

pub fn load_ak74_animation(
    animations: Res<AK74Animations>,
    mut players_query: Query<&mut AnimationPlayer>,
    mut current_animation: Local<AK74AnimationsList>,
    mut player_character_query: Query<(&Player, &AnimationEntityLink)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {

    for (player_entity, animation_entity_link) in &mut player_character_query.iter_mut() {
        if let Ok(mut animation_player) = players_query.get_mut(animation_entity_link.0) {
            keyboard_input.get_just_pressed().into_iter().for_each(|key_code| *current_animation = AK74AnimationsList::from(key_code));
    
            if mouse_input.pressed(MouseButton::Left) {
                // Continuously set the animation to 4 while the mouse button is held
                *current_animation = AK74AnimationsList::SHOOT;
            }
        
            if mouse_input.just_released(MouseButton::Left) {
                // Stop looping or switch animation when left-click is released
                *current_animation = AK74AnimationsList::IDLE; // Reset or change animation on release
            }
    
            if keyboard_input.just_released(KeyCode::KeyW) {
                *current_animation = AK74AnimationsList::IDLE;
            }
            
            if *current_animation != AK74AnimationsList::IDLE && animation_player.is_finished() {
                *current_animation = AK74AnimationsList::IDLE;
            }
    
            let animation: &mut AnimationPlayer = animation_player.play_with_transition(
                animations.0[*current_animation as usize].clone_weak(), 
                Duration::from_millis(100), // transition duration
            );
    
            if *current_animation == AK74AnimationsList::WALK || *current_animation == AK74AnimationsList::IDLE {
                    animation.repeat();
            }
    
            if *current_animation == AK74AnimationsList::SHOOT {
                    animation.repeat();
                    animation.set_speed(1.0); 
            }
        
        }
    }
}