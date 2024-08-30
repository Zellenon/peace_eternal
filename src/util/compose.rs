use avian3d::prelude::ExternalImpulse;
use bevy::{
    math::{Quat, Vec3},
    prelude::Transform,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};

pub fn with_translation(loc: Vec3, rot: Quat, scale: f32) -> ComponentTree {
    (Transform {
        translation: loc,
        rotation: rot,
        scale: Vec3::splat(scale),
    })
    .store()
}

pub fn instant_force(direction: Quat, force: f32) -> ComponentTree {
    (ExternalImpulse::new(direction * Vec3::NEG_Z * force)).store()
}
