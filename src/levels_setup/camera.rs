use bevy::{
    core::Name, core_pipeline::core_3d::Camera3dBundle, ecs::system::Commands, math::Vec3,
    transform::components::Transform,
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
                ..Default::default()
            },
            ..Default::default()
        })
        .insert((
            Name::new("FPSCamera"),
            FPSCamera,
            Shake::default(),
            AtmosphereCamera::default(),
            SmoothedTransform {
                smoothing: 0.5,
                do_translate: true,
                do_rotate: true,
                ..Default::default()
            },
        ));

    commands
        .spawn(Camera3dBundle::default())
        .insert((Name::new("TPSCamera"), TPSCamera))
        .insert((
            SmoothedTransform {
                smoothing: 0.2,
                do_translate: true,
                do_rotate: true,
                ..Default::default()
            },
            Shake::default(),
        ))
        .insert(AtmosphereCamera::default());
}
