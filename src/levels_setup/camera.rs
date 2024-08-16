use bevy::{
    core::Name,
    core_pipeline::{bloom::BloomSettings, core_3d::Camera3dBundle},
    ecs::system::Commands,
};
use bevy_atmosphere::plugin::AtmosphereCamera;

use crate::{
    character_control_systems::camera_controls::{FPSCamera, TPSCamera},
    util::{camera_shake::Shake, smoothing::SmoothedTransform},
};

pub fn setup_cameras(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            camera: bevy::render::camera::Camera {
                is_active: false,
                hdr: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert((
            Name::new("FPSCamera"),
            FPSCamera,
            Shake::default(),
            AtmosphereCamera::default(),
            BloomSettings::NATURAL,
            SmoothedTransform {
                smoothing: 15.,
                do_translate: true,
                do_rotate: true,
                rotation_mul: 5.,
                ..Default::default()
            },
        ));

    commands
        .spawn(Camera3dBundle {
            camera: bevy::prelude::Camera {
                hdr: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert((
            Name::new("TPSCamera"),
            TPSCamera,
            SmoothedTransform {
                smoothing: 7.,
                do_translate: true,
                do_rotate: true,
                ..Default::default()
            },
            BloomSettings::NATURAL,
            Shake::default(),
            AtmosphereCamera::default(),
        ));
}
