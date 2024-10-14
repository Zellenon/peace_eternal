use bevy::{
    core::Name,
    core_pipeline::{bloom::BloomSettings, core_3d::Camera3dBundle, tonemapping::Tonemapping},
    ecs::system::Commands,
    math::{Quat, Vec3},
    prelude::Transform,
};
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_composable::{
    app_impl::{ComplexSpawnable, ComponentTreeable},
    tree::ComponentTree,
};

use crate::{
    gameplay::controls::camera_controls::{FPSCamera, TPSCamera},
    util::{Shake, Smoothed},
};

fn basic_camera() -> ComponentTree {
    (
        Shake::default(),
        AtmosphereCamera::default(),
        BloomSettings::NATURAL,
    )
        .store()
}

pub fn setup_cameras(mut commands: Commands) {
    commands.compose(
        (
            Camera3dBundle {
                camera: bevy::render::camera::Camera {
                    is_active: false,
                    hdr: true,
                    ..Default::default()
                },
                tonemapping: Tonemapping::None,
                ..Default::default()
            },
            Name::new("FPSCamera"),
            FPSCamera,
        )
            .store()
            + Smoothed::<Transform, Vec3, "translation"> {
                speed: 20.,
                ..Default::default()
            }
            .store()
            + basic_camera(),
    );

    commands.compose(
        (
            Camera3dBundle {
                camera: bevy::prelude::Camera {
                    hdr: true,
                    ..Default::default()
                },
                tonemapping: Tonemapping::None,
                ..Default::default()
            },
            Name::new("TPSCamera"),
            TPSCamera,
            Smoothed::<Transform, Vec3, "translation"> {
                speed: 10.,
                ..Default::default()
            },
            Smoothed::<Transform, Quat, "rotation"> {
                speed: 18.,
                ..Default::default()
            },
        )
            .store()
            + basic_camera(),
    );
}
