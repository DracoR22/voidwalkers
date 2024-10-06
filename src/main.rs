use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy_kira_audio::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use voidhunt::{cubes::systems::spawn::spawn_cube_system, enemies::EnemiesPlugin, game::{blood::{cleanup_blood_effects, spawn_blood_mesh}, GamePlugin}, house::HousePlugin, player::{components::{Player, PlayerFirstPersonCamera}, PlayerPlugin}, ui::GameUIPugin, weapons::{states::CurrentWeapon, WeaponsPlugin}, window::WindowSetupPlugin};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::AlphaMode;

// Custom material for the muzzle flash
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct MuzzleFlashMaterial {
    #[uniform(0)]
   pub color: Color,
    #[texture(1)]
    #[sampler(2)]
   pub color_texture: Handle<Image>,
}

impl Material for MuzzleFlashMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/muzzle_flash.wgsl".into()
    }
}

#[derive(Component)]
pub struct HasFlash;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(LogPlugin {
        filter: "error".into(),
        level: bevy::log::Level::ERROR,
        ..default()
    }))
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
    .add_plugins(HanabiPlugin)
    .add_plugins(AudioPlugin)
    .add_plugins(MaterialPlugin::<MuzzleFlashMaterial>::default())
    .add_plugins(WindowSetupPlugin)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(GamePlugin)
    .add_plugins(HousePlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(EnemiesPlugin)
    .add_plugins(WeaponsPlugin)
    .add_plugins(GameUIPugin)
    .init_state::<CurrentWeapon>()
    .add_systems(Startup, spawn_cube_system)
    .add_systems(Update, setup_muzzle_flash)
    .add_systems(Update, update_muzzle_flash)
    .run();
}


#[derive(Component)]
pub struct MuzzleFlash {
    pub timer: Timer,
}

pub fn setup_muzzle_flash(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MuzzleFlashMaterial>>,
    asset_server: Res<AssetServer>,
    player_query: Query<(Entity, Option<&HasFlash>), With<Player>>,
) {
   
    let material = materials.add(MuzzleFlashMaterial {
        color: Color::rgba(5.0, 5.0, 5.0, 1.0), // Bright white with full alpha
        color_texture: asset_server.load("textures/muzzle-flash.png"),
    });

   if let Ok((player_entity, has_flash)) = player_query.get_single() {
    //  commands.entity(player_entity).with_children(|parent| {
    //       parent.spawn((
    //         MaterialMeshBundle {
    //             mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(200.2, 200.2)))),
    //             material,
    //             transform: Transform::from_xyz(0.2, 85.0, -67.0),
    //             visibility: Visibility::Visible,
    //             ..default()
    //         },
    //         MuzzleFlash {
    //             timer: Timer::from_seconds(1000.05, TimerMode::Once),
    //         },
    //     ));
    //  });
    
    commands.entity(player_entity).with_children(|parent| {
       if has_flash.is_none() {
        parent.spawn((
            SceneBundle {
                scene: asset_server.load("models/muzzle-flash.glb"),
                transform: Transform {
                    scale: Vec3::splat(300.0), 
                    translation: Vec3::new(10.2, 85.0, 7.0), 
                    rotation: Quat::from_rotation_y(std::f32::consts::PI), 
                    ..default()
                },
                visibility: Visibility::Visible,
                ..default()
            },
            MuzzleFlash {
                           timer: Timer::from_seconds(1000.05, TimerMode::Once),
                   },
        ));
        }
     });

     commands.entity(player_entity).insert(HasFlash);
   };
 
}

pub fn update_muzzle_flash(
    mut commands: Commands,
    time: Res<Time>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut param_set: ParamSet<(
        Query<(Entity, &mut MuzzleFlash, &mut Visibility, &mut Transform)>,
        Query<&Transform, (With<PlayerFirstPersonCamera>, Without<MuzzleFlash>)>,
    )>,
) {
    // Check if the left mouse button is pressed
    if mouse_input.just_pressed(MouseButton::Left) {
        // Get the camera transform
        let camera_transform = if let Ok(transform) = param_set.p1().get_single() {
            *transform
        } else {
            return; // Exit the function if there's no camera
        };

        // Iterate over all muzzle flash entities
        for (entity, mut muzzle_flash, mut visibility, mut transform) in param_set.p0().iter_mut() {
            // Reset the muzzle flash timer
            muzzle_flash.timer.reset();
            *visibility = Visibility::Visible;

            // Make the muzzle flash face towards the camera
            let direction_to_camera = camera_transform.translation - transform.translation;
            // transform.look_at(camera_transform.translation, Vec3::Y); 

            // Optionally adjust position, but don't reset it to a fixed point.
            // Only adjust this part if you want the muzzle flash to appear at the muzzle of the gun.
            transform.translation = Vec3::new(camera_transform.translation.x * 2.0, transform.translation.y, transform.translation.z * 2.0); // REMOVE OR MODIFY AS NEEDED
        }
    }

    // Update visibility based on the muzzle flash timer
    for (entity, mut muzzle_flash, mut visibility, _) in param_set.p0().iter_mut() {
        if muzzle_flash.timer.tick(time.delta()).just_finished() {
            *visibility = Visibility::Hidden;
        }
    }
}

#[derive(Asset, AsBindGroup, TypePath, Clone)]
struct LiquidMaterial {
    #[uniform(0)]
    time: f32,
}
