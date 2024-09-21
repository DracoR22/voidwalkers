use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ProjectileProperties {
   pub mesh: Handle<Mesh>,
   pub material: Handle<StandardMaterial>,
}