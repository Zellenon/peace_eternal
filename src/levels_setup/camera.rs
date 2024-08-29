use bevy::{
    core::Name,
    core_pipeline::{bloom::BloomSettings, core_3d::Camera3dBundle, tonemapping::Tonemapping},
    ecs::system::Commands,
};
use bevy_atmosphere::plugin::AtmosphereCamera;
use bevy_composable::tree::EntityCommandSet;
use bevy_composable::CT;
use bevy_composable::{app_impl::ComplexSpawnable, tree::ComponentTree};

use crate::{
    character_control_systems::camera_controls::{FPSCamera, TPSCamera},
    util::{camera_shake::Shake, smoothing::SmoothedTransform},
};

fn basic_camera() -> ComponentTree {
    CT!(
        Shake::default(),
        AtmosphereCamera::default(),
        BloomSettings::NATURAL
    )
}

pub fn setup_cameras(mut commands: Commands) {
    commands.spawn_complex(
        CT!(
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
                smoothing: 15.,
                do_translate: true,
                do_rotate: true,
                rotation_mul: 5.,
                ..Default::default()
            }
        ) + basic_camera(),
    );

    commands.spawn_complex(
        CT!(
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
            }
        ) + basic_camera(),
    );
}
