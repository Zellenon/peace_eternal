use bevy::{
    ecs::{component::Component, entity::Entity, query::Without, system::Query},
    math::{Quat, Vec3},
    prelude::{Event, EventReader, EventWriter, With},
    reflect::Reflect,
    transform::components::Transform,
};

use crate::{
    character_control_systems::camera_controls::Facing,
    levels_setup::IsPlayer,
    util::{camera_shake::TraumaEvent, smoothing::SmoothedTransform},
};

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
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

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
pub struct Recoil {
    pub arm: Entity,
    pub strength: f32,
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

pub(super) fn do_arm_recoil(
    mut recoils: EventReader<Recoil>,
    mut arms: Query<(&Arm, &mut Transform)>,
) {
    for Recoil { arm, strength } in recoils.read() {
        if let Ok((_arm, mut transform)) = arms.get_mut(*arm) {
            let mut offset = Transform::default().with_translation(Vec3::Z);
            offset.rotate_around(Vec3::ZERO, transform.rotation);
            transform.translation += offset.translation * (*strength) * 0.5;
            transform.rotation *= Quat::from_rotation_x(0.10) * (*strength);
        }
    }
}

pub fn do_shake_recoil(
    mut recoils: EventReader<Recoil>,
    arms: Query<&Arm>,
    player_entity: Query<Entity, With<IsPlayer>>,
    mut shake: EventWriter<TraumaEvent>,
) {
    let player_entity = player_entity.get_single().unwrap();
    recoils
        .read()
        .filter(|w| {
            if let Ok(Arm { parent, offset: _ }) = arms.get(w.arm) {
                parent.index() == player_entity.index()
            } else {
                false
            }
        })
        .for_each(|Recoil { arm: _, strength }| {
            shake.send((strength * 0.1).into());
        });
}
