use bevy::ecs::system::Query;
use bevy::prelude::Transform;
use bevy::{ecs::component::Component, reflect::Reflect};

#[derive(Component, Reflect)]
pub struct SmoothedTransform {
    pub smoothing: f32,
    pub goal: Transform,
    pub do_translate: bool,
    pub do_rotate: bool,
    pub do_scale: bool,
}

impl Default for SmoothedTransform {
    fn default() -> Self {
        Self {
            smoothing: 0.9,
            goal: Transform::default(),
            do_translate: false,
            do_rotate: false,
            do_scale: false,
        }
    }
}

pub fn smooth_movement(mut query: Query<(&mut Transform, &SmoothedTransform)>) {
    for (
        mut transform,
        &SmoothedTransform {
            smoothing,
            goal,
            do_translate,
            do_rotate,
            do_scale,
        },
    ) in query.iter_mut()
    {
        if do_translate {
            let delta = goal.translation - transform.translation;
            transform.translation = transform.translation + delta * smoothing;
        }
        // TODO: Rotation smoothing isn't working well
        if do_rotate {
            transform.rotation = transform.rotation.slerp(goal.rotation, smoothing);
        }
        if do_scale {
            let delta = goal.scale - transform.scale;
            transform.scale = transform.scale + delta * smoothing;
        }
    }
}
