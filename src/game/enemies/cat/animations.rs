use bevy::prelude::*;

use crate::common::link_animations::AnimationEntityLink;

use super::{components::Cat, resources::CatAnimations};

pub fn play_cat_animation(
    mut players_query: Query<&mut AnimationPlayer>,
    mut player_character_query: Query<(&Cat, &AnimationEntityLink)>,
    animations: Res<CatAnimations>,
) {
    for (cat, animation_entity_link) in &mut player_character_query.iter_mut() {
        if let Ok(mut animation_player) = players_query.get_mut(animation_entity_link.0) {
            let animation: &mut AnimationPlayer = animation_player.play(
                animations.0[0].clone_weak()
            );
    
            animation.repeat();
        } 
    }
}