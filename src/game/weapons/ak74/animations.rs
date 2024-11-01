use crate::{
    common::{link_animations::MultipleAnimationEntityLinks, states::CurrentWeapon},
    game::{
        player::components::Player,
        weapons::ak74::{components::AK74Component, resources::AK74Animations},
    },
};
use bevy::{prelude::*, utils::HashSet};
use std::time::Duration;

#[derive(PartialEq, Clone, Copy)]
pub enum AK74AnimationsList {
    Idle,
    Fire,
    ReloadFast,
    ReloadFull,
    Run,
    Walk,
}

impl Default for AK74AnimationsList {
    fn default() -> Self {
        Self::Idle
    }
}

impl From<&KeyCode> for AK74AnimationsList {
    fn from(key_code: &KeyCode) -> Self {
        match key_code {
            KeyCode::KeyW => AK74AnimationsList::Walk,
            KeyCode::KeyA => AK74AnimationsList::Walk,
            KeyCode::KeyS => AK74AnimationsList::Walk,
            KeyCode::KeyD => AK74AnimationsList::Walk,
            KeyCode::KeyR => AK74AnimationsList::ReloadFull,
            // do the same for all animations
            _ => AK74AnimationsList::Idle,
        }
    }
}

pub fn setup_ak74_animations(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AK74Animations(vec![
        asset_server.load("animations/saiga.glb#Animation0"),
        asset_server.load("animations/saiga.glb#Animation1"),
        asset_server.load("animations/saiga.glb#Animation2"),
        asset_server.load("animations/saiga.glb#Animation3"),
        asset_server.load("animations/saiga.glb#Animation4"),
        asset_server.load("animations/saiga.glb#Animation5"),
        // asset_server.load("animations/ak74.glb#Animation6"),
        // asset_server.load("animations/ak74.glb#Animation7"),
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
    ak74_query: Query<&AK74Component>,
    mut pressed_keys: Local<HashSet<KeyCode>>,
) {
    match state.get() {
        CurrentWeapon::AK74 => {
            for (player_entity, animation_entity_links) in player_character_query.iter_mut() {
                for &animation_entity in &animation_entity_links.0 {
                    if let Ok(mut animation_player) = players_query.get_mut(animation_entity) {
                        // println!("LOADING ANIMATIONS");
                        keyboard_input.get_just_pressed().into_iter().for_each(|key_code| {
                                 pressed_keys.insert(*key_code);
                                *current_animation = AK74AnimationsList::from(key_code)
                        });

                        
                        keyboard_input.get_just_released().for_each(|key_code| {
                            pressed_keys.remove(key_code);
                        });

                        // shooting
                        if let Ok(ak74) = ak74_query.get_single() {
                            if ak74.current_ammo > 0 {
                                if mouse_input.pressed(MouseButton::Left) {
                                    *current_animation = AK74AnimationsList::Fire;
                                }
                            } else {
                                if *current_animation == AK74AnimationsList::Fire {
                                    *current_animation = AK74AnimationsList::Idle;
                                }
                            }
                        }

                        if mouse_input.just_released(MouseButton::Left) {
                            // Stop looping or switch animation when left-click is released
                            *current_animation = AK74AnimationsList::Idle; // Reset or change animation on release
                        }

                        if *current_animation == AK74AnimationsList::Idle || *current_animation == AK74AnimationsList::Walk {
                            if pressed_keys.contains(&KeyCode::KeyW)
                                || pressed_keys.contains(&KeyCode::KeyA)
                                || pressed_keys.contains(&KeyCode::KeyS)
                                || pressed_keys.contains(&KeyCode::KeyD) {
                                *current_animation = AK74AnimationsList::Walk;
                            } else {
                                *current_animation = AK74AnimationsList::Idle;
                            }
                        }

                        
                        if *current_animation != AK74AnimationsList::Idle
                            && animation_player.is_finished()
                        {
                            *current_animation = AK74AnimationsList::Idle;
                        }

                        let animation: &mut AnimationPlayer = animation_player
                            .play_with_transition(
                                animations.0[*current_animation as usize].clone_weak(),
                                Duration::from_millis(100), // transition duration
                            );

                        if *current_animation == AK74AnimationsList::Idle {
                            animation.repeat();
                        }

                        if *current_animation == AK74AnimationsList::Walk {
                            animation.repeat();
                            animation.set_speed(2.0);
                        }

                        if *current_animation == AK74AnimationsList::Fire {
                            animation.repeat();
                            animation.set_speed(5.0);
                        }

                        if *current_animation == AK74AnimationsList::ReloadFull {
                            animation.set_speed(2.8);
                        }
                    }
                }
            }
        }

        // CurrentWeapon::Glock => {
        //     for (player_entity, animation_entity_links) in player_character_query.iter_mut() {
        //         for &animation_entity in &animation_entity_links.0 {
        //         if let Ok(mut animation_player) = players_query.get_mut(animation_entity) {
        //             // println!("LOADING ANIMATIONS");
        //             // keyboard_input.get_just_pressed().into_iter().for_each(|key_code| *current_animation = AK74AnimationsList::from(key_code));

        //             if keyboard_input.just_pressed(KeyCode::KeyR) {

        //                 *current_animation = AK74AnimationsList::GlockReloadFull
        //             }

        //             let animation: &mut AnimationPlayer = animation_player.play_with_transition(
        //                 animations.0[*current_animation as usize].clone_weak(),
        //                 Duration::from_millis(100), // transition duration
        //             );

        //             if *current_animation == AK74AnimationsList::GlockReloadFull {
        //                 println!("ANIMATION IS GLOCK LOOPING");
        //                 animation.repeat();
        //                 animation.set_speed(2.0);
        //         }

        //         }}
        //     }
        // }
        _ => (),
    }
}
