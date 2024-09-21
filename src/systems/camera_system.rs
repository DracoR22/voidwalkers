use bevy::prelude::*;
use bevy_flycam::FlyCam;

pub fn setup_cam(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 0.5),
            ..default()
        },
        FlyCam 
));}