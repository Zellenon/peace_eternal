use avian3d::prelude::ExternalImpulse;
use bevy::math::{Quat, Vec3};
use bevy::prelude::Transform;
use bevy_composable::tree::EntityCommandSet;
use bevy_composable::{tree::ComponentTree, CT};

pub fn with_translation(loc: Vec3, rot: Quat, scale: f32) -> ComponentTree {
    CT!(Transform {
        translation: loc,
        rotation: rot,
        scale: Vec3::splat(scale),
    })
}

pub fn instant_force(direction: Quat, force: f32) -> ComponentTree {
    CT!(ExternalImpulse::new(direction * Vec3::NEG_Z * force))
}
