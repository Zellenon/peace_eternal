use std::time::Duration;

use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::{With, Without},
        system::{Query, Res},
    },
    hierarchy::Children,
    math::{Quat, Vec3},
    reflect::Reflect,
    time::{Time, Timer},
    transform::components::Transform,
};
use leafwing_input_manager::action_state::ActionState;

use crate::{
    character_control_systems::{camera_controls::Facing, keyboard_receive::PlayerAction},
    levels_setup::IsPlayer,
};

#[derive(Event, Reflect)]
pub struct ActivateArm(pub Entity);

#[derive(Event, Reflect)]
pub struct ServoActivated(pub Entity);

#[derive(Component, Reflect)]
pub struct Arm {
    pub parent: Entity,
    pub offset: Transform,
}

impl Arm {
    pub fn new(entity: &Entity) -> Self {
        Self {
            parent: *entity,
            offset: Transform::default().with_translation(Vec3::new(0.4, 0.4, -0.5)),
        }
    }
}

#[derive(Reflect)]
pub enum FireMode {
    Manual,
    SemiAuto,
    FullAuto,
}

#[derive(Component, Reflect)]
pub struct Servo {
    pub firemode: FireMode,
    pub cooldown: Timer,
    pub wants_to_activate: bool,
}

impl Default for Servo {
    fn default() -> Self {
        Servo {
            firemode: FireMode::SemiAuto,
            cooldown: Timer::new(Duration::from_millis(750), bevy::time::TimerMode::Once),
            wants_to_activate: false,
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
        }
    }
}

// TODO: Run only when there are events
pub fn receive_arm_activation_events(
    mut activations: EventReader<ActivateArm>,
    arms: Query<(&Arm, &Children)>,
    mut activatables: Query<&mut Servo>,
) {
    for ActivateArm(arm_id) in activations.read() {
        if let Ok((arm, children)) = arms.get(*arm_id) {
            for child in children.iter() {
                if let Ok(mut activatable) = activatables.get_mut(*child) {
                    let should_activate = match activatable.firemode {
                        FireMode::Manual => activatable.cooldown.finished(),
                        FireMode::SemiAuto => true,
                        FireMode::FullAuto => true,
                    };
                    activatable.wants_to_activate =
                        should_activate || activatable.wants_to_activate;
                }
            }
        }
    }
}

pub fn tick_cooldowns(time: Res<Time>, mut cooldowns: Query<&mut Servo>) {
    for mut cooldown in cooldowns.iter_mut() {
        cooldown.cooldown.tick(time.delta());
    }
}

pub fn do_should_activate(
    mut events: EventWriter<ServoActivated>,
    mut activatables: Query<(Entity, &mut Servo)>,
) {
    for (entity, mut activatable) in activatables.iter_mut() {
        if activatable.cooldown.finished() && activatable.wants_to_activate {
            events.send(ServoActivated(entity));
            activatable.cooldown.reset();
            activatable.wants_to_activate = false;
        }
    }
}

// TODO: Use Aery to make this faster
pub fn player_arms_on_click(
    mut events: EventWriter<ActivateArm>,
    players: Query<(Entity, &ActionState<PlayerAction>), With<IsPlayer>>,
    arms: Query<(Entity, &Arm)>,
) {
    for (entity, action_state) in players.iter() {
        if action_state.just_pressed(&PlayerAction::Shoot) {
            arms.iter()
                .filter(|(_, arm)| arm.parent == entity)
                .for_each(|(e, _)| {
                    events.send(ActivateArm(e));
                });
        }
    }
}
