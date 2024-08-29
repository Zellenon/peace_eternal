use bevy::{
    asset::{Assets, Handle},
    core::Name,
    math::{UVec2, Vec2, Vec3, VectorSpace},
    prelude::{Image, ResMut},
};

use bevy_composable::{
    tree::{ComponentTree, EntityCommandSet},
    CT,
};

use bevy_hanabi::{
    Attribute, ColorOverLifetimeModifier, EffectAsset, ExprWriter, FlipbookModifier, Gradient,
    ImageSampleMapping, LinearDragModifier, OrientMode, OrientModifier, ParticleEffect,
    ParticleEffectBundle, ParticleGroupSet, ParticleTextureModifier, ScalarType,
    SetAttributeModifier, SetPositionSphereModifier, SetVelocitySphereModifier, ShapeDimension,
    SizeOverLifetimeModifier, Spawner,
};

use crate::fx::flags::DirectedFX;

pub fn smoke_puff(
    effects: &mut ResMut<Assets<EffectAsset>>,
    smoke: &Handle<Image>,
) -> ComponentTree {
    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(0.0, Vec3::splat(0.8).extend(0.02));
    color_gradient1.add_key(0.1, Vec3::splat(0.8).extend(0.05));
    // color_gradient1.add_key(0.5, Vec3::ONE.extend(0.05));
    color_gradient1.add_key(1.0, Vec3::splat(0.8).extend(0.));

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.0, Vec2::splat(0.01));
    size_gradient1.add_key(0.02, Vec2::splat(0.1));
    size_gradient1.add_key(0.1, Vec2::splat(1.));
    size_gradient1.add_key(0.3, Vec2::splat(1.5));
    size_gradient1.add_key(1.0, Vec2::splat(2.));

    let writer = ExprWriter::new();

    // Random age
    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    // Random lifespan
    let lifetime = writer.lit(2.5).uniform(writer.lit(3.5)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Add constant downward acceleration to simulate gravity
    // let accel = writer.lit(Vec3::Y * -13.).expr();
    // let update_accel = AccelModifier::new(accel);

    // Add drag to make particles slow down a bit after the initial explosion
    let drag = writer.lit(5.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(0.02).expr(),
        dimension: ShapeDimension::Volume,
    };

    let rotation = (writer.rand(ScalarType::Float) * writer.lit(std::f32::consts::TAU)).expr();
    let init_rotation = SetAttributeModifier::new(Attribute::F32_0, rotation);
    let rot_vel = SetAttributeModifier::new(
        Attribute::F32_1,
        (writer.lit(-1.).uniform(writer.lit(1.))).expr(),
    );
    let rotation_attr = (writer.attr(Attribute::F32_0)
        + writer.attr(Attribute::AGE) * writer.attr(Attribute::F32_1))
    .expr();

    // Give a bit of variation by randomizing the initial speed
    let direction = writer.add_property("direction", Vec3::Z.into());
    let direction = writer.prop(direction);
    let init_vel = SetVelocitySphereModifier {
        center: (writer.lit(0.09) * direction).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(30.) + writer.lit(4.)).expr(),
    };

    let sprite_index = writer
        .lit(0.)
        .uniform(writer.lit(5.))
        .cast(ScalarType::Int)
        .expr();
    let update_sprite_index = SetAttributeModifier::new(Attribute::SPRITE_INDEX, sprite_index);

    let trigger_spawn = Spawner::once(12.0.into(), false);
    let effect = EffectAsset::new(vec![256], trigger_spawn, writer.finish())
        .with_name("muzzle smoke")
        .init(init_pos)
        .init(init_vel)
        .init(init_age)
        .init(init_lifetime)
        .init(update_sprite_index)
        .init(init_rotation)
        .init(rot_vel)
        .render(FlipbookModifier {
            sprite_grid_size: UVec2::new(2, 2),
        })
        .update(update_drag)
        // .update(rotation_attr)
        // Currently the init pass doesn't run on cloned particles, so we have to use an update modifier
        // to init the lifetime of trails. This will overwrite the value each frame, so can only be used
        // for constant values.
        .render(ColorOverLifetimeModifier {
            gradient: color_gradient1.clone(),
        })
        .render(ParticleTextureModifier {
            texture: smoke.clone(),
            sample_mapping: ImageSampleMapping::Modulate,
        })
        .render(OrientModifier {
            mode: OrientMode::FaceCameraPosition,
            rotation: Some(rotation_attr),
        })
        .render(SizeOverLifetimeModifier {
            gradient: size_gradient1.clone(),
            screen_space_size: false,
        });
    // Tie together trail particles to make arcs. This way we don't need a lot of them, yet there's
    // a continuity between them.

    let effect1 = effects.add(effect);

    CT!(
        Name::new("smoke"),
        ParticleEffectBundle {
            effect: ParticleEffect::new(effect1.clone()),
            ..Default::default()
        },
        DirectedFX
    )
}
