use std::f32::consts::PI;

use bevy::asset::LoadState;
use bevy::core_pipeline::Skybox;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, TextureViewDescriptor, TextureViewDimension};

pub const CUBEMAPS: &[(&str, CompressedImageFormats)] = &[
    (
        "textures/skybox.png",
        CompressedImageFormats::NONE,
    ),
    // (
    //     "textures/Ryfjallet_cubemap_astc4x4.ktx2",
    //     CompressedImageFormats::ASTC_LDR,
    // ),
    // (
    //     "textures/Ryfjallet_cubemap_bc7.ktx2",
    //     CompressedImageFormats::BC,
    // ),
    // (
    //     "textures/Ryfjallet_cubemap_etc2.ktx2",
    //     CompressedImageFormats::ETC2,
    // ),
];


pub fn setup_skybox(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        DirectionalLight {
            illuminance: 32000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 2.0, 0.0).with_rotation(Quat::from_rotation_x(-PI / 4.)),
    ));
}

const CUBEMAP_SWAP_DELAY: f32 = 3.0;

pub fn cycle_cubemap_asset(
    time: Res<Time>,
    mut next_swap: Local<f32>,
    mut cubemap: ResMut<Cubemap>,
    asset_server: Res<AssetServer>,
    render_device: Res<RenderDevice>,
) {
    let now = time.elapsed_seconds();
    if *next_swap == 0.0 {
        *next_swap = now + CUBEMAP_SWAP_DELAY;
        return;
    } else if now < *next_swap {
        return;
    }
    *next_swap += CUBEMAP_SWAP_DELAY;

    let supported_compressed_formats =
        CompressedImageFormats::from_features(render_device.features());

    let mut new_index = cubemap.index;
    for _ in 0..CUBEMAPS.len() {
        new_index = (new_index + 1) % CUBEMAPS.len();
        if supported_compressed_formats.contains(CUBEMAPS[new_index].1) {
            break;
        }
        info!(
            "Skipping format which is not supported by current hardware: {:?}",
            CUBEMAPS[new_index]
        );
    }

    // Skip swapping to the same texture. Useful for when ktx2, zstd, or compressed texture support
    // is missing
    if new_index == cubemap.index {
        return;
    }

    cubemap.index = new_index;
    cubemap.image_handle = asset_server.load(CUBEMAPS[cubemap.index].0);
    cubemap.is_loaded = false;
}


use bevy::render::renderer::RenderDevice;
use bevy::render::texture::{CompressedImageFormats, Image};

#[derive(Resource)]
pub struct Cubemap {
   pub image_handle: Handle<Image>,
   pub is_loaded: bool,
   pub index: usize,
}

pub fn asset_loaded(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
    mut skyboxes: Query<&mut Skybox>,
) {
    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle) == LoadState::Loaded {
        info!("Swapping to {}...", CUBEMAPS[cubemap.index].0);
        let image = images.get_mut(&cubemap.image_handle).unwrap();
        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        for mut skybox in &mut skyboxes {
            skybox.image = cubemap.image_handle.clone();
        }

        cubemap.is_loaded = true;
    }
}

pub fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() * 0.5);
    }
}