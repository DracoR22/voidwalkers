use bevy::prelude::*;

use crate::common::entities::EntityType;

pub fn spawn_plaster_bullethole(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    world_coords: Vec3,
    rotation: Quat,
) {
    let bullethole_texture: Handle<Image> = asset_server.load("textures/bullethole_plaster.png");

    let quad_mesh = meshes.add(Mesh::from(shape::Quad {
        size: Vec2::new(1.0, 1.0),
        flip: false,
    }));

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(bullethole_texture.clone()),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    commands.spawn((PbrBundle {
        mesh: quad_mesh,
        material,
        transform: Transform {
            translation: world_coords,
            scale: Vec3::splat(10.0), 
            rotation,
            ..Default::default()
        },
        ..Default::default()
    },));
}
