use bevy::prelude::*;
use std::time::Duration;
use crate::{common::link_animations::MultipleAnimationEntityLinks, game::weapons::{components::AK74Component, resources::AK74Animations, states::CurrentWeapon}, player::components::Player};

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
            KeyCode::KeyE => AK74AnimationsList::DRAW,
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
        asset_server.load("animations/ak74.glb#Animation0"), // add more animations
        asset_server.load("animations/ak74.glb#Animation1"),
        asset_server.load("animations/ak74.glb#Animation2"),
        asset_server.load("animations/ak74.glb#Animation3"),
        asset_server.load("animations/ak74.glb#Animation4"),
        asset_server.load("animations/ak74.glb#Animation5"),
        asset_server.load("animations/ak74.glb#Animation6"),
        asset_server.load("animations/ak74.glb#Animation7")
    ]));
}

pub fn load_ak74_animation(
    animations: Res<AK74Animations>,
    mut players_query: Query<&mut AnimationPlayer>,
    mut current_animation: Local<AK74AnimationsList>,
    mut player_character_query: Query<(&Player, &MultipleAnimationEntityLinks)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    state: Res<State<CurrentWeapon>>,
    ak74_query: Query<&AK74Component>
) {
    match state.get() {
        CurrentWeapon::AK74 => {
            for (player_entity, animation_entity_links) in &mut player_character_query.iter_mut() {
                for &animation_entity in &animation_entity_links.0 {
                if let Ok(mut animation_player) = players_query.get_mut(animation_entity) {
                    keyboard_input.get_just_pressed().into_iter().for_each(|key_code| *current_animation = AK74AnimationsList::from(key_code));
            
                    // shoot
                    if let Ok(ak74) = ak74_query.get_single() {
                        if ak74.current_ammo > 0 {
                            if mouse_input.pressed(MouseButton::Left) {
                                *current_animation = AK74AnimationsList::SHOOT;
                            }
                        
                        }
                    }
            
                    if mouse_input.just_released(MouseButton::Left) {
                        // Stop looping or switch animation when left-click is released
                        *current_animation = AK74AnimationsList::IDLE; // Reset or change animation on release
                    }
            
                    if keyboard_input.just_released(KeyCode::KeyW)
                    || keyboard_input.just_released(KeyCode::KeyA)
                    || keyboard_input.just_released(KeyCode::KeyS)
                    || keyboard_input.just_released(KeyCode::KeyD) {
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
                            animation.set_speed(2.0); 
                    }

                    if *current_animation == AK74AnimationsList::RELOADFULL {
                        animation.set_speed(1.8); 
                    }
                
                }}
            }
        }

        _ => ()
    }
}