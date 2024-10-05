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


#[derive(Component)]
pub struct MuzzleFlash {
    pub timer: Timer,
}

pub fn setup_muzzle_flash(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MuzzleFlashMaterial>>,
    asset_server: Res<AssetServer>,
) {
   
    let material = materials.add(MuzzleFlashMaterial {
        color: Color::rgba(5.0, 5.0, 5.0, 1.0), // Bright white with full alpha
        color_texture: asset_server.load("textures/muzzle-flash.png"),
    });

 // Muzzle flash (using a flat quad)
 commands.spawn((
    MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(20.2, 20.2)))),
        material,
        transform: Transform::from_xyz(700.0, 100.0, 0.0),
        visibility: Visibility::Hidden,
        ..default()
    },
    MuzzleFlash {
        timer: Timer::from_seconds(0.05, TimerMode::Once),
    },
));
}

pub fn update_muzzle_flash(
    mut commands: Commands,
    time: Res<Time>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut param_set: ParamSet<(
        Query<(Entity, &mut MuzzleFlash, &mut Visibility, &mut Transform)>,
        Query<&Transform, (With<Camera>, Without<MuzzleFlash>)>
    )>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        // let camera_transform = camera_query.single();

        // println!("TRANSFORM: {:?}", camera_transform.translation);
        for (entity, mut muzzle_flash, mut visibility, mut transform) in param_set.p0().iter_mut() {
            muzzle_flash.timer.reset();
            *visibility = Visibility::Visible;

            // update translation
            transform.translation = Vec3::new(700.0, 10.0, 0.0)
        }
    }

    for (entity, mut muzzle_flash, mut visibility, mut transform) in param_set.p0().iter_mut() {
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
