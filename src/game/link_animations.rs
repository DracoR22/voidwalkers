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