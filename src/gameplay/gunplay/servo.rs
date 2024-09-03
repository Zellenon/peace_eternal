use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        system::{Query, Res},
    },
    hierarchy::Children,
    math::{Quat, Vec3},
    prelude::GlobalTransform,
    reflect::Reflect,
    time::{Time, Timer},
};
use leafwing_input_manager::action_state::ActionState;
use std::time::Duration;

use super::{arms::Arm, dummy_gun::Barrel};
use crate::gameplay::{controls::keyboard_receive::PlayerAction, levels_setup::IsPlayer};

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct Servo {
    pub firemode: FireMode,
    pub cooldown: Timer,
    pub wants_to_activate: bool,
}

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
pub struct ArmServo(pub Entity, pub bool);

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
pub struct ServoActivated(pub Entity);

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
pub struct DirectedServoActivated {
    pub servo: Entity,
    pub barrel: Entity,
    pub location: Vec3,
    pub rotation: Quat,
}

#[derive(Eq, PartialOrd, Ord, Component, Reflect, Clone, Debug, PartialEq)]
pub enum FireMode {
    Manual,
    SemiAuto,
    FullAuto,
}

impl Servo {
    pub fn new(firemode: FireMode, cooldown: u64) -> Self {
        Self {
            firemode,
            cooldown: Timer::new(Duration::from_millis(cooldown), bevy::time::TimerMode::Once),
            wants_to_activate: false,
        }
    }
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

impl ArmServo {
    pub fn new(e: &Entity) -> Self {
        Self(*e, true)
    }

    pub fn disarm(self) -> Self {
        Self(self.0, false)
    }
}

impl From<Entity> for ArmServo {
    fn from(value: Entity) -> Self {
        Self::new(&value)
    }
}

impl From<&Entity> for ArmServo {
    fn from(value: &Entity) -> Self {
        Self::new(&value)
    }
}

// TODO: Run only when there are events
pub fn receive_servo_arming_events(
    mut activations: EventReader<ArmServo>,
    mut activatables: Query<&mut Servo>,
) {
    for ArmServo(servo_id, do_arm) in activations.read() {
        if let Ok(mut activatable) = activatables.get_mut(*servo_id) {
            if *do_arm {
                activatable.wants_to_activate = match activatable.firemode {
                    FireMode::Manual => activatable.cooldown.finished(),
                    FireMode::SemiAuto => true,
                    FireMode::FullAuto => true,
                };
            } else {
                activatable.wants_to_activate = false;
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
    mut servos: Query<(Entity, &mut Servo)>,
) {
    for (entity, mut servo) in servos.iter_mut() {
        if servo.cooldown.finished() && servo.wants_to_activate {
            events.send(ServoActivated(entity));
            servo.cooldown.reset();
            if servo.firemode != FireMode::FullAuto {
                servo.wants_to_activate = false;
            }
        }
    }
}

pub fn do_directed_servos(
    mut activations: EventReader<ServoActivated>,
    mut directed_activations: EventWriter<DirectedServoActivated>,
    servos: Query<&Children, With<Servo>>,
    barrels: Query<(Entity, &GlobalTransform), With<Barrel>>,
) {
    for ServoActivated(entity) in activations.read() {
        if let Ok(children) = servos.get(*entity) {
            let (barrel, position) = children
                .iter()
                .filter_map(|w| barrels.get(*w).ok())
                .next()
                .unwrap();
            let (_, rot, loc) = position.to_scale_rotation_translation();

            directed_activations.send(DirectedServoActivated {
                servo: *entity,
                barrel,
                location: loc,
                rotation: rot,
            });
        }
    }
}

// TODO: Use Aery to make this faster
pub fn player_servos_on_click(
    mut events: EventWriter<ArmServo>,
    players: Query<(Entity, &ActionState<PlayerAction>), With<IsPlayer>>,
    arms: Query<(Entity, &Arm, &Children)>,
) {
    if let Ok((player_entity, action_state)) = players.get_single() {
        if action_state.just_pressed(&PlayerAction::Shoot) {
            arms.iter()
                .filter(|(_, arm, _)| arm.parent == player_entity)
                .for_each(|(_, _, children)| {
                    children.iter().for_each(|e| {
                        events.send(e.into());
                    });
                });
        } else if action_state.just_released(&PlayerAction::Shoot) {
            arms.iter()
                .filter(|(_, arm, _)| arm.parent == player_entity)
                .for_each(|(_, _, children)| {
                    children.iter().for_each(|e| {
                        events.send(ArmServo::new(e).disarm());
                    });
                });
        }
    }
}
