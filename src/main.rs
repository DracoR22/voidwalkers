use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy_kira_audio::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use voidhunt::{cubes::systems::spawn::spawn_cube_system, enemies::EnemiesPlugin, game::{blood::{cleanup_blood_effects, spawn_blood_mesh}, GamePlugin}, house::HousePlugin, player::{components::{Player, PlayerFirstPersonCamera}, PlayerPlugin}, ui::GameUIPugin, weapons::{glock::spawn::MuzzleFlash, states::CurrentWeapon, WeaponsPlugin}, window::WindowSetupPlugin};
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
    .add_systems(Startup, setup_muzzle_flash)
    .add_systems(Update, update_muzzle_flash)
    .run();
}


pub fn setup_muzzle_flash(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MuzzleFlashMaterial>>,
    asset_server: Res<AssetServer>,
    player_query: Query<(Entity, Option<&HasFlash>), With<Player>>,
) {
   
    let material = materials.add(MuzzleFlashMaterial {
        color: Color::rgba(1.0, 1.0, 1.0, 1.0), // Bright white with full alpha
        color_texture: asset_server.load("textures/MuzzleFlash.png"),
    });

    commands.spawn((
            MaterialMeshBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(30.2, 30.2)))),
                material,
                transform: Transform::from_xyz(700.0, 10.0, 0.0),
                visibility: Visibility::Visible,
                ..default()
            },
            MuzzleFlash {
                timer: Timer::from_seconds(1000.05, TimerMode::Once),
            },
    ));
    
// Spawn a 2D UI node that holds the muzzle flash
commands.spawn((
    NodeBundle {
        style: Style {
            width: Val::Px(50.0),
            height: Val::Px(50.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            
            ..default()
        },
        visibility: Visibility::Hidden,  
        ..default()
    },
    MuzzleFlash {
        timer: Timer::from_seconds(0.05, TimerMode::Once), 
    },
)).with_children(|parent| {
    parent.spawn(ImageBundle {
        image: UiImage {
            texture: asset_server.load("textures/MuzzleFlash.png"),
            ..default()
        },

         ..default()
    });
});
    
 
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
            // let direction_to_camera = camera_transform.translation - transform.translation;
            // transform.look_at(camera_transform.translation, Vec3::Y); 

            // Optionally adjust position, but don't reset it to a fixed point.
            // Only adjust this part if you want the muzzle flash to appear at the muzzle of the gun.
            // transform.translation = Vec3::new(camera_transform.translation.x * 2.0, transform.translation.y, transform.translation.z * 2.0); // REMOVE OR MODIFY AS NEEDED
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
