use bevy::prelude::*;
use bevy::prelude::AlphaMode;
use bevy_hanabi::prelude::*;

use super::components::BloodEffectLifetime;

 pub fn spawn_blood(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    position_x: f32,
    position_y: f32,
    position_z: f32,
    parent: Option<Entity>,
    asset_server: Res<AssetServer>
 ) {

     let texture_handle: Handle<Image> = asset_server.load("images/cloud.png");
    
    let mut gradient = Gradient::new();
    // Very dark red at the start (almost brown-red for realistic blood)
    gradient.add_key(0.0, Vec4::new(0.3, 0.0, 0.0, 1.0));  // Dark brownish-red at spawn
    // Intermediate phase with less saturation and slight transparency
    gradient.add_key(0.5, Vec4::new(0.2, 0.0, 0.0, 0.7));  // Midway fading to darker red
    // Fully transparent and almost black at the end of the lifetime
    gradient.add_key(1.0, Vec4::new(0.1, 0.0, 0.0, 0.0));  // Nearly black and fully transparent
    

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec2::splat(0.1));  // Start with small particles
    size_gradient1.add_key(0.3, Vec2::splat(0.1));  // Hold size for a bit
    size_gradient1.add_key(1.0, Vec2::splat(0.0));   // Shrink to nothing at the end
    

    let writer = ExprWriter::new();

    // Give a bit of variation by randomizing the age per particle. This will
    // control the starting color and starting size of particles.
    let age = writer.lit(0.).uniform(writer.lit(0.2)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime = writer.lit(0.8).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Simulate gravity
    let gravity = writer.lit(Vec3::new(0.0, -50.0, 0.0)).expr();
    let update_gravity = AccelModifier::new(gravity);

    // Lifetime for trails
    let init_lifetime_trails =
        SetAttributeModifier::new(Attribute::LIFETIME, writer.lit(0.1).expr());

    // Add constant downward acceleration to simulate gravity
    let accel = writer.lit(Vec3::Y * -32.).expr();
    let update_accel = AccelModifier::new(accel);

    // Add drag to make particles slow down a bit after the initial explosion
    let drag = writer.lit(5.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(0.1).expr(),
        dimension: ShapeDimension::Volume,
    };

    // Give a bit of variation by randomizing the initial speed
    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(50.) + writer.lit(100.)).expr(),
    };

    // Clear the trail velocity so trail particles just stay in place as they fade
    // away
    let init_vel_trail =
        SetAttributeModifier::new(Attribute::VELOCITY, writer.lit(Vec3::ZERO).expr());

    let lead = ParticleGroupSet::single(0);
    let trail = ParticleGroupSet::single(1);

    let color = writer.rand(VectorType::VEC4F).pack4x8unorm();
    let init_color = SetAttributeModifier::new(Attribute::COLOR, color.expr());

    let alpha_cutoff =
    ((writer.time() * writer.lit(2.)).sin() * writer.lit(0.3) + writer.lit(0.4)).expr();

    let effect = EffectAsset::new(
        // 2k lead particles, with 32 trail particles each
        vec![90006, 90006 * 32],
        Spawner::rate(90006.0.into()),
        writer.finish(),
    )
    .with_name("firework")
    .with_alpha_mode(bevy_hanabi::AlphaMode::Mask(alpha_cutoff))
    .init(init_pos)
    .init(init_vel)
    .init(init_age)
    .init(init_lifetime)
    .init(init_color)
    .update_groups(CloneModifier::new(1.0 / 64.0, 1), lead)
    .update_groups(update_drag, lead)
    .render(ParticleTextureModifier {
        texture: texture_handle,
        sample_mapping: ImageSampleMapping::ModulateOpacityFromR,
    })
    .update_groups(init_lifetime_trails, trail)
    // .update_groups(init_vel_trail, trail)
    .render_groups(
        ColorOverLifetimeModifier {
            gradient: gradient.clone(),
        },
        lead,
    )
    .render_groups(
        SizeOverLifetimeModifier {
            gradient: size_gradient1.clone(),
            screen_space_size: false,
        },
        lead,
    )
    .render_groups(
        ColorOverLifetimeModifier {
            gradient: gradient,
        },
        trail,
    )
    .render_groups(
        SizeOverLifetimeModifier {
            gradient: size_gradient1,
            screen_space_size: false,
        },
        trail,
    )
    // Tie together trail particles to make arcs. This way we don't need a lot of them, yet there's
    // a continuity between them.
    .render_groups(RibbonModifier, trail);

    let effect1 = effects.add(effect);

    let blood_effect = (
        Name::new("blood"),
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect1),
            transform:  Transform::from_xyz(position_x, position_y, position_z),
            ..Default::default()
        },
        BloodEffectLifetime {
            timer: Timer::from_seconds(2.0, TimerMode::Once), // Adjust the duration as needed
        },
    );

    match parent {
        Some(parent_entity) => {
            // Spawn as a child of the provided entity
            commands.entity(parent_entity).with_children(|parent| {
                parent.spawn(blood_effect);
            });
        }
        None => {
            // Spawn as a standalone entity
            commands.spawn(blood_effect);
        }
    }

   
}

pub fn cleanup_blood_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut BloodEffectLifetime)>,
) {
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_blood_mesh(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Load the blood splatter texture
    let blood_texture = asset_server.load("images/blood-splatter.png");

    // Create a quad (2D plane) for the blood splatter
    let quad_mesh = meshes.add(Mesh::from(shape::Quad { size: Vec2::new(1.0, 1.0), flip: false }));

    // Create the material with the blood texture
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(blood_texture.clone()),
        alpha_mode: AlphaMode::Blend,
        ..Default::default()
    });

    // Spawn the blood splatter quad with the texture applied
    commands.spawn((
        PbrBundle {
            mesh: quad_mesh,
            material,
            transform: Transform {
                translation: Vec3::new(700.0, 1.0, 0.0),
                scale: Vec3::new(100.0, 100.0, 100.0),  // Adjust scale for the blood size
                rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),  // Rotate to face camera/surface
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}