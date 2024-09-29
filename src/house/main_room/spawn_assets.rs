use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
       // Add a directional light
    //    commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         shadows_enabled: true,
    //         color: Color::rgb(1.0, 0.8, 0.6),
    //         illuminance: 800.0,
    //         ..default()
    //     },
    //     transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
    //     ..default()
    // });

      // ambient light
    //   commands.insert_resource(AmbientLight {
    //     color: Color::rgb(1.0, 0.8, 0.6),
    //     brightness: 10.0,
    // });

       // point light
       commands
       .spawn(PointLightBundle {
           // transform: Transform::from_xyz(5.0, 8.0, 2.0),
           transform: Transform::from_xyz(730.0, 226.0, -395.0),
           point_light: PointLight {
               intensity: 100_000_000_0.0,
               range: 100_000_000_000_000.0,
               color: Color::rgb(1.0, 0.8, 0.6),
               shadows_enabled: true,
               ..default()
           },
           ..default()
       })
       .with_children(|builder| {
           builder.spawn(SceneBundle {
            scene: asset_server.load("models/lamp.glb#Scene0"),
            transform: Transform {
                rotation: Quat::from_rotation_y(std::f32::consts::PI / 2.0), 
                scale: Vec3::splat(20.0),
                translation: Vec3::new(0.0, 0.0, 64.0),
                ..default()
            },
               ..default()
           });
       });

       // point light 2
       commands
       .spawn(PointLightBundle {
           // transform: Transform::from_xyz(5.0, 8.0, 2.0),
           transform: Transform::from_xyz(1780.0, 236.0, -165.0),
           point_light: PointLight {
               intensity: 100_000_000_0.0,
               range: 100_000_000_000.0,
               color: Color::rgb(1.0, 0.8, 0.6),
               shadows_enabled: true,
               ..default()
           },
           ..default()
       })
       .with_children(|builder| {
           builder.spawn(SceneBundle {
            scene: asset_server.load("models/lamp.glb#Scene0"),
            transform: Transform {
                rotation: Quat::from_rotation_y(-std::f32::consts::PI / 2.0), 
                scale: Vec3::splat(20.0),
                translation: Vec3::new(0.0, 0.0, -64.0),
                ..default()
            },
               ..default()
           });
       });


     // Bright warm light to simulate muzzle flash
    //  commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 50000.0, // High intensity for a quick flash
    //         color: Color::rgb(1.0, 0.8, 0.6), // Warm color like a fire/explosion
    //         range: 500.0, // Adjust range to fit the flash
    //         radius: 100.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(700.0, 1.0, 0.0), // Position the light at the gun's muzzle (transform from the gun or player)
    //     ..default()
    // });

    
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

    // sofa
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/sofa.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(1130.0, 10.0, -500.0),
            scale: Vec3::splat(125.0),
            // rotation: Quat::from_rotation_y(std::f32::consts::PI),
            ..default()
        },
        ..default()
    });

    // xmas tree
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/xmas_tree.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(1720.0, 1.0, 200.0),
            scale: Vec3::splat(100.0),
            rotation: Quat::from_rotation_y(std::f32::consts::PI),
            ..default()
        },
        ..default()
    });

    // presents
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/white_present.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(1720.0, 10.0, 300.0),
            scale: Vec3::splat(1.0),
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

     // wood window
     commands.spawn(SceneBundle {
        scene: asset_server.load("models/wooden_window.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(1215.0, 5.0, -49.0),
            scale: Vec3::splat(3.0),
            rotation: Quat::from_rotation_y(-std::f32::consts::PI / 2.0),
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
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load("models/wood_slab_coffee_table.glb#Scene0"),
    //     transform: Transform {
    //         translation: Vec3::new(1350.0, 1.0, -60.0),
    //         scale: Vec3::splat(160.0),
    //         rotation: Quat::from_rotation_y(std::f32::consts::PI),
    //         ..default()
    //     },
    //     ..default()
    // });
}