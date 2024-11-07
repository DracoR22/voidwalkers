use std::{fs::File, io::Write};


use bevy::{pbr::NotShadowCaster, prelude::*, tasks::IoTaskPool, utils::Duration};
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
    .add_plugins((DefaultPlugins
        .set(LogPlugin {
            filter: "error".into(),
            level: bevy::log::Level::ERROR,
            ..default()
        }), DefaultPlugins.set(AssetPlugin {
            watch_for_changes_override: Some(true),
            ..default()
        }))
    )

    .register_type::<ComponentA>()
    .register_type::<ComponentB>()
    .register_type::<ResourceA>()

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


#[derive(Component, Reflect, Default)]
#[reflect(Component)] // this tells the reflect derive to also reflect component behaviors
struct ComponentA {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ComponentB {
    pub value: String,
    #[reflect(skip_serializing)]
    pub _time_since_startup: Duration,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct ResourceA {
    pub score: u32,
}

impl FromWorld for ComponentB {
    fn from_world(world: &mut World) -> Self {
        let time = world.resource::<Time>();
        ComponentB {
            _time_since_startup: time.elapsed(),
            value: "Default Value".to_string(),
        }
    }
}


// The initial scene file will be loaded below and not change when the scene is saved
const SCENE_FILE_PATH: &str = "scenes/load_scene_example.scn.ron";

// The new, updated scene data will be saved here so that you can see the changes
const NEW_SCENE_FILE_PATH: &str = "scenes/load_scene_example-new.scn.ron";

fn load_scene_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawning a DynamicSceneRoot creates a new entity and spawns new instances
    // of the given scene's entities as children of that entity.
    // Scenes can be loaded just like any other asset.
    commands.spawn(DynamicSceneBundle {
        scene: asset_server.load(SCENE_FILE_PATH),
        ..default()
    });
}


pub fn log_system(
    query: Query<(Entity, &ComponentA), Changed<ComponentA>>,
    res: Option<Res<ResourceA>>,
) {
    for (entity, component_a) in &query {
        info!("  Entity({})", entity.index());
        info!(
            "    ComponentA: {{ x: {} y: {} }}\n",
            component_a.x, component_a.y
        );
    }
    if let Some(res) = res {
        if res.is_added() {
            info!("  New ResourceA: {{ score: {} }}\n", res.score);
        }
    }
}

fn save_scene_system(world: &mut World) {
    // Scenes can be created from any ECS World.
    // You can either create a new one for the scene or use the current World.
    // For demonstration purposes, we'll create a new one.
    let mut scene_world = World::new();

    // The `TypeRegistry` resource contains information about all registered types (including components).
    // This is used to construct scenes, so we'll want to ensure that our previous type registrations
    // exist in this new scene world as well.
    // To do this, we can simply clone the `AppTypeRegistry` resource.
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    scene_world.insert_resource(type_registry);

    let mut component_b = ComponentB::from_world(world);
    component_b.value = "hello".to_string();
    scene_world.spawn((
        component_b,
        ComponentA { x: 1.0, y: 2.0 },
        Transform::IDENTITY,
    ));
    scene_world.spawn(ComponentA { x: 3.0, y: 4.0 });
    scene_world.insert_resource(ResourceA { score: 1 });

    // With our sample world ready to go, we can now create our scene using DynamicScene or DynamicSceneBuilder.
    // For simplicity, we will create our scene using DynamicScene:
    let scene = DynamicScene::from_world(&scene_world);

    // Scenes can be serialized like this:
    let type_registry = world.resource::<AppTypeRegistry>();
    let serialized_scene = scene.serialize_ron(type_registry).unwrap();

    // Showing the scene in the console
    info!("{}", serialized_scene);

    // Writing the scene to a new file. Using a task to avoid calling the filesystem APIs in a system
    // as they are blocking
    // This can't work in WASM as there is no filesystem access
    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/{NEW_SCENE_FILE_PATH}"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}