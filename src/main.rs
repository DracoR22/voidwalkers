use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy::log::LogPlugin;
use bevy_atmosphere::plugin::AtmosphereSkyBox;
use bevy_kira_audio::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use descent::game::map::MapPlugin;
use descent::{cubes::systems::spawn::spawn_cube_system, effects::EffectsPlugin, game::GamePlugin, house::HousePlugin, ui::GameUIPugin, window::WindowSetupPlugin};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

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
    .add_plugins(DefaultPlugins
        .set(LogPlugin {
            filter: "error".into(),
            level: bevy::log::Level::ERROR,
            ..default()
        })
    )
    .add_plugins(FrameTimeDiagnosticsPlugin::default())
    .add_plugins(HanabiPlugin)
    .add_plugins(AudioPlugin)
    .add_plugins(MapPlugin)
    // .add_plugins(MaterialPlugin::<SkyboxMaterial>::default())
    .add_plugins(MaterialPlugin::<MuzzleFlashMaterial>::default())
    .add_plugins(WindowSetupPlugin)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(GamePlugin)
    .add_plugins(HousePlugin)
    .add_plugins(GameUIPugin)
    .add_systems(Startup, spawn_cube_system)
    .add_plugins(EffectsPlugin)
    // .add_systems(
    //     Update,
    //     (
    //         cycle_cubemap_asset,
    //         asset_loaded.after(cycle_cubemap_asset),
    //         animate_light_direction,
    //     ),
    // )
    // .add_systems(Startup, setup_skybox)
    .run();
}

fn update_sky_position(
    atmosphere_query: Query<&AtmosphereSkyBox>
) {
    let atmosphere = atmosphere_query.get_single().unwrap();
    
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle: Handle<Image> = asset_server.load("textures/NightSky_Top.png");
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(100000.0, 100000.0, 100000.0)),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle),
                unlit: true,
                cull_mode: None,
                
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(20.0)),
            ..default()
        },
        NotShadowCaster,
    ));
}





#[derive(Asset, AsBindGroup, TypePath, Clone)]
struct LiquidMaterial {
    #[uniform(0)]
    time: f32,
}
