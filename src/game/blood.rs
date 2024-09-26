use bevy::prelude::*;
use bevy_hanabi::prelude::*;

 pub fn spawn_blood(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    position_x: f32,
    position_y: f32,
    position_z: f32
 ) {
    
    let mut gradient = Gradient::new();
    // Very dark red at the start (almost brown-red for realistic blood)
    gradient.add_key(0.0, Vec4::new(0.3, 0.0, 0.0, 1.0));  // Dark brownish-red at spawn
    // Intermediate phase with less saturation and slight transparency
    gradient.add_key(0.5, Vec4::new(0.2, 0.0, 0.0, 0.7));  // Midway fading to darker red
    // Fully transparent and almost black at the end of the lifetime
    gradient.add_key(1.0, Vec4::new(0.1, 0.0, 0.0, 0.0));  // Nearly black and fully transparent
    

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec2::splat(0.05));  // Start with small particles
    size_gradient1.add_key(0.3, Vec2::splat(0.05));  // Hold size for a bit
    size_gradient1.add_key(1.0, Vec2::splat(0.0));   // Shrink to nothing at the end
    

    let writer = ExprWriter::new();

    // Give a bit of variation by randomizing the age per particle. This will
    // control the starting color and starting size of particles.
    let age = writer.lit(0.).uniform(writer.lit(0.2)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime = writer.lit(0.8).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Lifetime for trails
    let init_lifetime_trails =
        SetAttributeModifier::new(Attribute::LIFETIME, writer.lit(0.2).expr());

    // Add constant downward acceleration to simulate gravity
    let accel = writer.lit(Vec3::Y * -16.).expr();
    let update_accel = AccelModifier::new(accel);

    // Add drag to make particles slow down a bit after the initial explosion
    let drag = writer.lit(4.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(0.01).expr(),
        dimension: ShapeDimension::Volume,
    };

    // Give a bit of variation by randomizing the initial speed
    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(40.) + writer.lit(80.)).expr(),
    };

    // Clear the trail velocity so trail particles just stay in place as they fade
    // away
    let init_vel_trail =
        SetAttributeModifier::new(Attribute::VELOCITY, writer.lit(Vec3::ZERO).expr());

    let lead = ParticleGroupSet::single(0);
    let trail = ParticleGroupSet::single(1);

    let effect = EffectAsset::new(
        // 2k lead particles, with 32 trail particles each
        vec![50048, 50048 * 32],
        Spawner::rate(50048.0.into()),
        writer.finish(),
    )
    .with_name("firework")
    .init(init_pos)
    .init(init_vel)
    .init(init_age)
    .init(init_lifetime)
    .update_groups(CloneModifier::new(1.0 / 64.0, 1), lead)
    // .update_groups(update_drag, lead)
    .update_groups(update_accel, lead)
    // Currently the init pass doesn't run on cloned particles, so we have to use an update modifier
    // to init the lifetime of trails. This will overwrite the value each frame, so can only be used
    // for constant values.
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

    commands.spawn((
        Name::new("firework"),
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect1),
            transform:  Transform::from_xyz(position_x, position_y, position_z),
            ..Default::default()
        },
    ));
}