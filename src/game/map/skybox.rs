use bevy::prelude::*;

pub fn setup_skybox(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut shaders: ResMut<Assets<Shader>>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    let cubemap_faces = [
        "textures/NightSky_Right.png",
        "textures/NightSky_Left.png",
        "textures/NightSky_Top.png",
        "textures/NightSky_Bottom.png",
        "textures/NightSky_Front.png",
        "textures/NightSky_Back.png",
    ];

    // let mut cubemap_textures = Vec::new();
}