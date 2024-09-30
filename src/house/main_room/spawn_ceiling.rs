use bevy::prelude::*;

pub fn spawn_ceiling(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
  let mesh = mesh_assets.add(Cuboid::new(1500.0, 10.0, 1120.0));

   // Load the textures
   let albedo_texture_handle = asset_server.load("textures/Ceiling_ALB.png"); // Base color (albedo)
   let normal_texture_handle = asset_server.load("textures/Ceiling_NRM.png"); // Normal map
   let rma_texture_handle = asset_server.load("textures/Ceiling_RMA.png");    // Roughness/Metallic/AO map

   let material_handle = material_assets.add(StandardMaterial {
    base_color_texture: Some(albedo_texture_handle), // Albedo map
    normal_map_texture: Some(normal_texture_handle), // Normal map
    metallic_roughness_texture: Some(rma_texture_handle),
    ..default()
});

  commands.spawn(PbrBundle {
    mesh: mesh.clone(),
    material: material_handle,
    transform: Transform::from_translation(Vec3::new(1060.0, 350.0, -20.0)), 
    ..default()
});
}