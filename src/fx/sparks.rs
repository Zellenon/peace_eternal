use bevy::{
    asset::Assets,
    core::Name,
    math::{Vec2, Vec3, Vec4},
    prelude::ResMut,
};

use bevy_composable::{
    tree::{ComponentTree, EntityCommandSet},
    CT,
};

use bevy_hanabi::{
    Attribute, CloneModifier, ColorOverLifetimeModifier, EffectAsset, ExprWriter, Gradient,
    LinearDragModifier, ParticleEffect, ParticleEffectBundle, ParticleGroupSet, RibbonModifier,
    ScalarType, SetAttributeModifier, SetPositionSphereModifier, SetVelocitySphereModifier,
    ShapeDimension, SizeOverLifetimeModifier, Spawner,
};

use crate::fx::flags::DirectedFX;

pub fn basic_sparks(effects: &mut ResMut<Assets<EffectAsset>>) -> ComponentTree {
    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 0.6));
    color_gradient1.add_key(0.3, Vec4::new(4.0, 4.0, 0.0, 0.6));
    color_gradient1.add_key(0.6, Vec4::new(4.0, 0.0, 0.0, 0.3));
    color_gradient1.add_key(1.0, Vec4::new(4.0, 0.0, 0.0, 0.0));

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec2::splat(0.03));
    size_gradient1.add_key(0.3, Vec2::splat(0.03));
    size_gradient1.add_key(1.0, Vec2::splat(0.0));

    let writer = ExprWriter::new();

    // Give a bit of variation by randomizing the age per particle. This will
    // control the starting color and starting size of particles.
    let age = writer.lit(0.).uniform(writer.lit(0.09)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    // Give a bit of variation by randomizing the lifetime per particle
    let lifetime = writer.lit(0.1).uniform(writer.lit(0.14)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Lifetime for trails
    let init_lifetime_trails =
        SetAttributeModifier::new(Attribute::LIFETIME, writer.lit(0.06).expr());

    // Add constant downward acceleration to simulate gravity
    // let accel = writer.lit(Vec3::Y * -13.).expr();
    // let update_accel = AccelModifier::new(accel);

    // Add drag to make particles slow down a bit after the initial explosion
    let drag = writer.lit(20.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(0.02).expr(),
        dimension: ShapeDimension::Volume,
    };

    // Give a bit of variation by randomizing the initial speed
    // let init_vel = SetVelocitySphereModifier {
    //     center: writer.lit(Vec3::Z).expr(),
    //     speed: (writer.rand(ScalarType::Float) * writer.lit(20.) + writer.lit(60.)).expr(),
    // };
    let direction = writer.add_property("direction", Vec3::Z.into());
    let direction = writer.prop(direction);
    let init_vel = SetVelocitySphereModifier {
        center: (writer.lit(0.06) * direction).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(60.) + writer.lit(10.)).expr(),
    };

    // Clear the trail velocity so trail particles just stay in place as they fade
    // away
    let init_vel_trail =
        SetAttributeModifier::new(Attribute::VELOCITY, writer.lit(Vec3::ZERO).expr());

    let lead = ParticleGroupSet::single(0);
    let trail = ParticleGroupSet::single(1);

    let trigger_spawn = Spawner::once(10.0.into(), false);
    let effect = EffectAsset::new(
        // 2k lead particles, with 32 trail particles each
        vec![1024, 1024 * 32],
        trigger_spawn,
        writer.finish(),
    )
    .with_name("muzzle sparks")
    .init(init_pos)
    .init(init_vel)
    .init(init_age)
    .init(init_lifetime)
    .update_groups(CloneModifier::new(1.0 / 64.0, 1), lead)
    .update_groups(update_drag, lead)
    // .update_groups(update_accel, lead)
    // Currently the init pass doesn't run on cloned particles, so we have to use an update modifier
    // to init the lifetime of trails. This will overwrite the value each frame, so can only be used
    // for constant values.
    .update_groups(init_lifetime_trails, trail)
    .update_groups(init_vel_trail, trail)
    .render_groups(
        ColorOverLifetimeModifier {
            gradient: color_gradient1.clone(),
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
            gradient: color_gradient1,
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

    CT!(
        Name::new("sparks"),
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect1.clone()),
            ..Default::default()
        },
        DirectedFX
    )
}
