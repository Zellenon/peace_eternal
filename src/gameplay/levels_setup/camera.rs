use bevy::{
    core::Name,
    core_pipeline::{bloom::BloomSettings, core_3d::Camera3dBundle, tonemapping::Tonemapping},
    ecs::system::Commands,
};
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_composable::{
    app_impl::{ComplexSpawnable, ComponentTreeable},
    tree::ComponentTree,
};

use crate::{
    gameplay::controls::camera_controls::{FPSCamera, TPSCamera},
    util::{Shake, SmoothedTransform},
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
            SmoothedTransform {
                smoothing: 25.,
                do_translate: true,
                do_rotate: true,
                rotation_mul: 2.,
                ..Default::default()
            },
        )
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
            SmoothedTransform {
                smoothing: 7.,
                do_translate: true,
                do_rotate: true,
                ..Default::default()
            },
        )
            .store()
            + basic_camera(),
    );
}
