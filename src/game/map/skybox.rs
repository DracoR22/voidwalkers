use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use light_consts::lux::AMBIENT_DAYLIGHT;

use super::{CycleTimer, Sun};

const SUN_INTENSITY_SCALE: f32 = 0.07;
const DAY_NIGHT_CYCLE_DURATION: f32 = 300.0;

pub fn daylight_cycle(
    mut atmosphere: AtmosphereMut<Nishita>,
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
    mut timer: ResMut<CycleTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        let t = (time.elapsed_seconds_wrapped() / DAY_NIGHT_CYCLE_DURATION) * std::f32::consts::TAU;
        atmosphere.sun_position = Vec3::new(0., t.sin(), t.cos());

        if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
            light_trans.rotation = Quat::from_rotation_x(-t);
            directional.illuminance = t.sin().max(0.0).powf(2.0) * AMBIENT_DAYLIGHT * SUN_INTENSITY_SCALE;
        }
    }
}

pub fn setup_environment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Our Sun
    commands.spawn((
        DirectionalLightBundle {
            ..Default::default()
        },
        Sun, // Marks the light as Sun
    ));

    // Simple transform shape just for reference
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(StandardMaterial::from(Color::rgb(0.8, 0.8, 0.8))),
        ..Default::default()
    });

    // X axis
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
        material: materials.add(StandardMaterial::from(Color::rgb(0.8, 0.0, 0.0))),
        transform: Transform::from_xyz(1., 0., 0.),
        ..Default::default()
    });

    // Y axis
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
        material: materials.add(StandardMaterial::from(Color::rgb(0.0, 0.8, 0.0))),
        transform: Transform::from_xyz(0., 1., 0.),
        ..Default::default()
    });

    // Z axis
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
        material: materials.add(StandardMaterial::from(Color::rgb(0.0, 0.0, 0.8))),
        transform: Transform::from_xyz(0., 0., 1.),
        ..Default::default()
    });
}