use bevy::{
    ecs::{component::Component, entity::Entity, query::Without, system::Query},
    math::{Quat, Vec3},
    reflect::Reflect,
    transform::components::Transform,
};

use crate::{
    character_control_systems::camera_controls::Facing, util::smoothing::SmoothedTransform,
};

#[derive(Component, Reflect)]
pub struct Arm {
    pub parent: Entity,
    pub offset: Transform,
}

impl Arm {
    pub fn new(entity: &Entity) -> Self {
        Self {
            parent: *entity,
            offset: Transform::default().with_translation(Vec3::new(0.35, 0.25, -0.6)),
        }
    }
}

pub(crate) fn update_arm_position(
    mut arms: Query<(&mut SmoothedTransform, &Arm)>,
    parents: Query<(&Transform, &Facing), Without<Arm>>,
) {
    for (mut arm_transform, Arm { parent, offset }) in arms.iter_mut() {
        if let Ok((parent_global, facing)) = parents.get(*parent) {
            arm_transform.goal = parent_global.mul_transform(*offset);
            arm_transform.goal.look_to(facing.forward, Vec3::Y);
            let pitch_axis = arm_transform.goal.left();
            arm_transform.goal.rotate_around(
                parent_global.translation + -0.5 * Vec3::Y,
                Quat::from_axis_angle(*pitch_axis, facing.pitch_angle),
            );
        }
    }
}
