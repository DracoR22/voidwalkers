use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationEntityLink(pub Entity);

pub fn get_top_parent(mut curr_entity: Entity, parent_query: &Query<&Parent>) -> Entity {
   //Loop up all the way to the top parent
   loop {
       if let Ok(parent) = parent_query.get(curr_entity) {
           curr_entity = parent.get();
       } else {
           break;
       }
   }
   curr_entity
}

// this function is used to find an entity with an animation player inside of another entity
pub fn link_animations(
   player_query: Query<Entity, Added<AnimationPlayer>>,
   parent_query: Query<&Parent>,
   animations_entity_link_query: Query<&AnimationEntityLink>,
   mut commands: Commands,
) {
   // Get all the Animation players which can be deep and hidden in the heirachy
   for entity in player_query.iter() {
       let top_entity = get_top_parent(entity, &parent_query);
 
       // If the top parent has an animation config ref then link the player to the config
       if animations_entity_link_query.get(top_entity).is_ok() {
           warn!("Problem with multiple animationsplayers for the same top parent");
       } else {
           commands
               .entity(top_entity)
               .insert(AnimationEntityLink(entity.clone()));
       }
   }
}

#[derive(Component)]
pub struct MultipleAnimationEntityLinks(pub Vec<Entity>);

// this implementation let us have multiple animation players inside the same entity, why? because a player can own multiple weapons!
pub fn link_multiple_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    mut animations_entity_link_query: Query<&mut MultipleAnimationEntityLinks>,
    mut commands: Commands,
) {
  // Get all the Animation players that may be deep and hidden in the hierarchy
  for entity in player_query.iter() {
    let top_entity = get_top_parent(entity, &parent_query);
    
    // Check if the top parent already has AnimationEntityLinks
    if let Ok(mut links) = animations_entity_link_query.get_mut(top_entity) {
        // If it does, add the new animation player entity
        links.0.push(entity);
    } else {
        // Otherwise, insert a new AnimationEntityLinks with the current animation player
        commands.entity(top_entity).insert(MultipleAnimationEntityLinks(vec![entity]));
    }
}
}
