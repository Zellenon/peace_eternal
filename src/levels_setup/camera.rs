use bevy::{
    core::Name, core_pipeline::core_3d::Camera3dBundle, ecs::system::Commands, math::Vec3,
    transform::components::Transform,
};
use bevy_atmosphere::plugin::AtmosphereCamera;

use crate::{
    character_control_systems::camera_controls::{FPSCamera, TPSCamera},
    util::smoothing::SmoothedTransform,
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
        .insert((Name::new("FPSCamera"), FPSCamera));

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 16.0, 40.0)
                .looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
            ..Default::default()
        })
        .insert((Name::new("TPSCamera"), TPSCamera))
        .insert(SmoothedTransform {
            smoothing: 0.2,
            do_translate: true,
            do_rotate: true,
            ..Default::default()
        })
        .insert(AtmosphereCamera::default());
}
