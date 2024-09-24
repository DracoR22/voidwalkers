use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
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

    
    // emisive ball
    // let material = materials.add(StandardMaterial {
    //     emissive: Color::rgb_linear(5.32, 2.0, 13.99) * 700.0,
    //     ..default()
    // });
    // let mesh = meshes.add(Sphere::new(50.0).mesh().ico(5).unwrap());

    // commands.spawn((
    //     PbrBundle {
    //         mesh: mesh.clone(),
    //         material,
    //         transform: Transform::from_xyz(700.0, 240.0, 0.0),
            
    //         ..default()
    //     },
    // )) 
    // .insert(RigidBody::Dynamic)
    // .insert(Collider::ball(35.0));

    // bean bag chair
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/bean_bag_chair.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(600.0, 0.0, 0.0), // Position floors in a grid
            scale: Vec3::splat(50.0),
            ..default()
        },
        ..default()
    });

    // victorian sofa
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/victorian_lounge_sofa.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(1720.0, 90.0, -90.0),
            scale: Vec3::splat(190.0),
            rotation: Quat::from_rotation_y(std::f32::consts::PI),
            ..default()
        },
        ..default()
    });

    // plant pot
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/indoor_plant.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(1720.0, 1.0, 150.0),
            scale: Vec3::splat(20.0),
            rotation: Quat::from_rotation_y(std::f32::consts::PI),
            ..default()
        },
        ..default()
    });

    // wood window
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/wooden_window.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(1260.0, 1.0, -40.0),
            scale: Vec3::splat(3.0),
            rotation: Quat::from_rotation_y(std::f32::consts::PI),
            ..default()
        },
        ..default()
    });

    // floor lamp
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/floor-lamp.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(270.0, 1.0, -430.0),
            scale: Vec3::splat(100.0),
            rotation: Quat::from_rotation_y(std::f32::consts::PI),
            ..default()
        },
        ..default()
    });

    // coffee table
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/wood_slab_coffee_table.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(1350.0, 1.0, -60.0),
            scale: Vec3::splat(160.0),
            rotation: Quat::from_rotation_y(std::f32::consts::PI),
            ..default()
        },
        ..default()
    });

    // cat
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/cat.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(1350.0, 1.0, -360.0),
            scale: Vec3::splat(3.0),
            rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
            ..default()
        },
        ..default()
    });
}