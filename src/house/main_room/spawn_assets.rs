use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::house::components::DoorComponent;

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
           transform: Transform::from_xyz(760.0, 286.0, -520.0),
           point_light: PointLight {
               intensity: 100_000_000_0.0,
               range: 100_000_000_00.0,
               color: Color::rgb(1.0, 0.8, 0.6),
            //    shadows_enabled: true,
               ..default()
           },
           ..default()
       });

       commands.spawn(SceneBundle {
        scene: asset_server.load("models/lamp.glb#Scene0"),
        transform: Transform {
            rotation: Quat::from_rotation_y(std::f32::consts::PI / 2.0), 
            scale: Vec3::splat(20.0),
            translation: Vec3::new(730.0, 226.0, -395.0),
            ..default()
        },
           ..default()  
       });

       // point light 2
       commands
       .spawn(PointLightBundle {
           // transform: Transform::from_xyz(5.0, 8.0, 2.0),
           transform: Transform::from_xyz(1400.0, 286.0, -510.0),
           point_light: PointLight {
               intensity: 100_000_000_0.0,
               range: 100_000.0,
               color: Color::rgb(1.0, 0.8, 0.6),
            //    shadows_enabled: true,
               ..default()
           },
           ..default()
       });

       // door frame
       commands.spawn(SceneBundle {
        scene: asset_server.load("models/door_frame.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(720.0, 0.0, -140.0), 
            scale: Vec3::new(0.37, 0.4, 0.37),
            ..default()
        },
        ..default()
    });

       // door
       commands.spawn(SceneBundle {
        scene: asset_server.load("models/wooden_door.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(720.0, 135.0, -140.0), 
            scale: Vec3::new(155.0, 135.0, 155.0),
            rotation: Quat::from_rotation_y(std::f32::consts::PI / 2.0), 
            ..default()
        },
        ..default()
    }).insert(DoorComponent {
        is_opening: false,
        timer: Timer::from_seconds(0.2, TimerMode::Once),
        is_opened: false,
        initial_pos: Vec3::new(720.0, 135.0, -140.0), 
    });

    // light switch
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/light_switch.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(723.0, 155.0, -275.0), 
            scale: Vec3::splat(210.0),
            // rotation: Quat::from_rotation_y(std::f32::consts::PI / 2.0), 
            ..default()
        },
        ..default()
    });



    //    .with_children(|builder| {
    //        builder.spawn(SceneBundle {
    //         scene: asset_server.load("models/lamp.glb#Scene0"),
    //         transform: Transform {
    //             rotation: Quat::from_rotation_y(-std::f32::consts::PI / 2.0), 
    //             scale: Vec3::splat(20.0),
    //             translation: Vec3::new(0.0, 0.0, -64.0),
    //             ..default()
    //         },
    //            ..default()
    //        });
    //    });

    
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

    // cube
    

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

    // antique table
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/antique_table.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(760.0, 2.0, -420.0),
            scale: Vec3::splat(137.0),
            rotation: Quat::from_rotation_y(std::f32::consts::PI / 2.0),
            ..default()
        },
        ..default()
    });

    // sofa
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/sofa.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(1130.0, 3.0, -500.0),
            scale: Vec3::splat(125.0),
            // rotation: Quat::from_rotation_y(std::f32::consts::PI),
            ..default()
        },
        ..default()
    });

    // table lamp
    commands.spawn(SceneBundle {
        scene: asset_server.load("models/table-lamp.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(1300.0, 3.0, -1000.0),
            scale: Vec3::splat(137.0),
            ..default()
        },
        ..default()
    });

    // xmas tree
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load("models/xmas_tree.glb#Scene0"),
    //     transform: Transform {
    //         translation: Vec3::new(1720.0, 1.0, 200.0),
    //         scale: Vec3::splat(100.0),
    //         rotation: Quat::from_rotation_y(std::f32::consts::PI),
    //         ..default()
    //     },
    //     ..default()
    // });

    // // presents
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load("models/white_present.glb#Scene0"),
    //     transform: Transform {
    //         translation: Vec3::new(1720.0, 10.0, 300.0),
    //         scale: Vec3::splat(1.0),
    //         ..default()
    //     },
    //     ..default()
    // });


     // wood window next to sofa
     commands.spawn(SceneBundle {
        scene: asset_server.load("models/wooden_window.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(1215.0, 5.0, -50.0),
            scale: Vec3::splat(3.0),
            rotation: Quat::from_rotation_y(-std::f32::consts::PI / 2.0),
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

pub fn open_door(
    mut door_query: Query<(&mut Transform, & mut DoorComponent)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
) {

   if let Ok((mut transform, mut door)) = door_query.get_single_mut() {
       let hinge_offset = Vec3::new(1.0, 0.0, 1.2); 
       let translation_speed_factor = 0.4;

       if keyboard_input.just_pressed(KeyCode::KeyE) && !door.is_opening {
          door.is_opening = true;
          door.timer.reset();
       }

       if door.is_opening {
           door.timer.tick(time.delta());

           let progress = door.timer.elapsed_secs() / door.timer.duration().as_secs_f32();
           let translation_progress = (door.timer.elapsed_secs() * translation_speed_factor) / door.timer.duration().as_secs_f32();


           if door.is_opened {
            let target_rotation = Quat::from_rotation_y(std::f32::consts::PI / 2.0); // Fully opened position (90 degrees)
           let initial_rotation = Quat::from_rotation_y(0.0); // Closed position
           
           let new_rotation = initial_rotation.slerp(target_rotation, progress);
           
           transform.translation = transform.translation.lerp(door.initial_pos, translation_progress);
           transform.rotation = new_rotation;
           
           } else {
            let target_rotation = Quat::from_rotation_y(0.0); // Fully opened position (90 degrees)
           let initial_rotation = Quat::from_rotation_y(std::f32::consts::PI / 2.0); // Closed position
           
           let new_rotation = initial_rotation.slerp(target_rotation, progress);

           // Apply rotation and adjust position for hinge
           transform.rotation = new_rotation;
           transform.translation = transform.translation + hinge_offset * (3.5 - progress);

           }

           // If the timer finishes, stop the animation
           if door.timer.finished() {
            door.is_opening = false;
            door.is_opened = !door.is_opened;
        }

       }
   }
}