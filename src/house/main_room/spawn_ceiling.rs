use bevy::prelude::*;

pub fn spawn_ceiling(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
  let mesh = mesh_assets.add(Cuboid::new(1500.0, 10.0, 1120.0));

  let texture_handle = asset_server.load("textures/Ceiling_ALB.png");

  let material_handle = material_assets.add(StandardMaterial {
      base_color_texture: Some(texture_handle),
      ..default()
  });

  commands.spawn(PbrBundle {
    mesh: mesh.clone(),
    material: material_handle,
    transform: Transform::from_translation(Vec3::new(1060.0, 350.0, -20.0)), 
    ..default()
});
}