use bevy::{
    ecs::{component::Component, entity::Entity, query::Without, system::Query},
    math::{Quat, Vec3},
    reflect::Reflect,
    transform::components::Transform,
};

use crate::character_control_systems::camera_controls::Facing;

#[derive(Component, Reflect)]
pub struct Arm {
    parent: Entity,
    offset: Transform,
}

impl Arm {
    pub fn new(entity: &Entity) -> Self {
        Self {
            parent: *entity,
            offset: Transform::default().with_translation(Vec3::new(0.4, 0.4, -0.5)),
        }
    }
}

pub(crate) fn update_arm_position(
    mut arms: Query<(&mut Transform, &Arm)>,
    parents: Query<(&Transform, &Facing), Without<Arm>>,
) {
    for (mut arm_transform, Arm { parent, offset }) in arms.iter_mut() {
        if let Ok((parent_global, facing)) = parents.get(*parent) {
            *arm_transform = parent_global.mul_transform(*offset);
            arm_transform.look_to(facing.forward, Vec3::Y);
            let pitch_axis = arm_transform.left();
            arm_transform.rotate_around(
                parent_global.translation + -0.5 * Vec3::Y,
                Quat::from_axis_angle(*pitch_axis, facing.pitch_angle),
            );
            println!("Updating transform to {}", parent_global.translation);
        }
    }
}
