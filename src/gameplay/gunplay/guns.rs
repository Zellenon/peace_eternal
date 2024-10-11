use bevy::{
    ecs::component::Component,
    math::{Quat, Vec3},
    prelude::{Entity, Event, EventReader, EventWriter, Parent, Query, With},
    reflect::Reflect,
};

use crate::gameplay::inventory::{
    components::Inventory,
    swapping::{HeldBy, HoldingInventoryItem},
};

use super::{
    arms::{Arm, Recoil},
    dummy_gun::DummyGun,
    servo::DirectedServoActivated,
};

pub type BarrelPos = (Vec3, Quat);

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct Gun {
    pub barrel_pos: BarrelPos,
}

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
pub struct FireGun {
    pub entity: Entity,
    pub position: Vec3,
    pub orientation: Quat,
}

#[derive(Event, Debug, Reflect)]
pub struct DummyMirror(pub DirectedServoActivated, pub Entity);

#[derive(Event, Debug, Reflect)]
pub struct RecoilMirror(pub Recoil);

pub fn dummy_activations_to_inventory_guns(
    mut events: EventReader<DirectedServoActivated>,
    mut mirror: EventWriter<DummyMirror>,
    arms: Query<(&Arm, &HoldingInventoryItem)>,
    dummy_gun: Query<&Parent, With<DummyGun>>,
    inventories: Query<&Inventory>,
) {
    events.read().for_each(|activation| {
        dummy_gun
            .get(activation.servo)
            .ok()
            .map(|parent| arms.get(parent.get()).ok())
            .flatten()
            .map(|(arm, held_item)| inventories.get(arm.parent).ok().zip(held_item.held_slot))
            .flatten()
            .map(|(inventory, slot)| {
                inventory
                    .slots
                    .get(slot)
                    .map(|w| w.contents)
                    .flatten()
                    .iter()
                    .for_each(|item| {
                        mirror.send(DummyMirror(activation.clone(), *item));
                    });
            });
    });
}

pub fn unmirror_gun_activations(
    mut mirrors: EventReader<DummyMirror>,
    mut events: EventWriter<DirectedServoActivated>,
) {
    for DummyMirror(original, new_entity) in mirrors.read() {
        events.send({
            let mut original = original.clone();
            original.servo = *new_entity;
            original
        });
    }
}

pub fn gun_recoil_to_arm(
    mut reader: EventReader<Recoil>,
    mut writer: EventWriter<RecoilMirror>,
    guns: Query<&HeldBy>,
) {
    for recoil in reader.read() {
        if let Ok(gun) = guns.get(recoil.arm) {
            writer.send(RecoilMirror(Recoil {
                arm: gun.0,
                strength: recoil.strength,
            }));
        }
    }
}

pub fn gun_recoil_to_arm2(mut reader: EventReader<RecoilMirror>, mut writer: EventWriter<Recoil>) {
    for RecoilMirror(recoil) in reader.read() {
        writer.send(recoil.clone());
    }
}
