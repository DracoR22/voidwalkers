use bevy::prelude::*;

pub fn spawn_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
       // Add a directional light
       commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            color: Color::rgb(1.0, 0.9, 0.7),
            illuminance: 200.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("models/bean_bag_chair.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(600.0, 0.0, 0.0), // Position floors in a grid
            scale: Vec3::splat(50.0),
            ..default()
        },
        ..default()
    });
}