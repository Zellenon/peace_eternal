use bevy::ecs::system::{Query, Res};
use bevy::prelude::Transform;
use bevy::time::Time;
use bevy::{ecs::component::Component, reflect::Reflect};

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct SmoothedTransform {
    pub smoothing: f32,
    pub goal: Transform,
    pub do_translate: bool,
    pub do_rotate: bool,
    pub do_scale: bool,
    pub rotation_mul: f32,
}

impl Default for SmoothedTransform {
    fn default() -> Self {
        Self {
            smoothing: 0.9,
            goal: Transform::default(),
            do_translate: false,
            do_rotate: false,
            do_scale: false,
            rotation_mul: 1.,
        }
    }
}

pub fn smooth_movement(time: Res<Time>, mut query: Query<(&mut Transform, &SmoothedTransform)>) {
    for (
        mut transform,
        &SmoothedTransform {
            smoothing,
            goal,
            do_translate,
            do_rotate,
            do_scale,
            rotation_mul,
        },
    ) in query.iter_mut()
    {
        if do_translate {
            let delta = goal.translation - transform.translation;
            transform.translation =
                transform.translation + delta * (smoothing * time.delta().as_secs_f32());
        }
        // TODO: Rotation smoothing isn't working well
        if do_rotate {
            transform.rotation = transform.rotation.slerp(
                goal.rotation,
                (smoothing * time.delta().as_secs_f32() * rotation_mul).min(1.),
            );
        }
        if do_scale {
            let delta = goal.scale - transform.scale;
            transform.scale = transform.scale + delta * (smoothing * time.delta().as_secs_f32());
        }
    }
}
