use bevy::prelude::{Entity, EventWriter, Query, With};
use leafwing_input_manager::prelude::ActionState;

use crate::gameplay::{
    inventory::{
        components::Inventory,
        pickups::SpawnPickup,
        swapping::{ChangeHeldInventoryItem, HoldingInventoryItem},
    },
    levels_setup::IsPlayer,
    Arm,
};

use super::keyboard_receive::InventoryAction;

pub fn receive_hotbar_command(
    mut events: EventWriter<ChangeHeldInventoryItem>,
    players: Query<(Entity, &ActionState<InventoryAction>, &Inventory), With<IsPlayer>>,
    arms: Query<(Entity, &Arm, &HoldingInventoryItem)>,
) {
    for (player_entity, actions, inventory) in players.iter() {
        let hotbar_destination: Option<Option<usize>> =
            if actions.just_pressed(&InventoryAction::ToSlot1) {
                Some(Some(0))
            } else if actions.just_pressed(&InventoryAction::ToSlot2) {
                Some(Some(1))
            } else if actions.just_pressed(&InventoryAction::ToSlot3) {
                Some(Some(2))
            } else if actions.just_pressed(&InventoryAction::ToSlot4) {
                Some(Some(3))
            } else if actions.just_pressed(&InventoryAction::ToSlot5) {
                Some(Some(4))
            } else {
                let (_, _, current_held_item) = arms
                    .iter()
                    .filter(|(_entity, arm, _held_item)| arm.parent == player_entity)
                    .next()
                    .unwrap();
                let (current_held_item, last_held_item) = (
                    current_held_item.held.map(|(a, b)| a).unwrap_or(0),
                    current_held_item.last_held_item,
                );
                let inventory_size = inventory.slots.len();
                if actions.just_pressed(&InventoryAction::PreviousWeapon) {
                    Some(Some((current_held_item - 1) % inventory_size))
                } else if actions.just_pressed(&InventoryAction::NextWeapon) {
                    Some(Some((current_held_item + 1) % inventory_size))
                } else if actions.just_pressed(&InventoryAction::LastUsedWeapon) {
                    Some(Some(last_held_item))
                } else if actions.just_pressed(&InventoryAction::Holster) {
                    Some(None)
                } else {
                    None
                }
            };
        if let Some(destination) = hotbar_destination {
            let arm_entity = arms
                .iter()
                .filter(|(_entity, arm, _held_item)| arm.parent == player_entity)
                .next()
                .unwrap()
                .0;
            events.send(ChangeHeldInventoryItem {
                arm: arm_entity,
                slot: destination,
            });
        }
    }
}

pub fn player_drop_held_weapon(
    mut change_slot: EventWriter<ChangeHeldInventoryItem>,
    mut weapon_drops: EventWriter<SpawnPickup>,
    players: Query<(Entity, &ActionState<InventoryAction>, &mut Inventory), With<IsPlayer>>,
    arms: Query<(Entity, &Arm, &HoldingInventoryItem)>,
) {
    for (player_entity, actions, inventory) in players.iter() {
        if actions.just_pressed(&InventoryAction::DropWeapon) {
            for (arm_entity, arm, held_item) in arms.iter().filter(|w| w.1.parent == player_entity)
            {
                change_slot.send(ChangeHeldInventoryItem {
                    arm: arm_entity,
                    slot: None,
                });
            }
        }
        let hotbar_destination: Option<Option<usize>> =
            if actions.just_pressed(&InventoryAction::ToSlot1) {
                Some(Some(0))
            } else if actions.just_pressed(&InventoryAction::ToSlot2) {
                Some(Some(1))
            } else if actions.just_pressed(&InventoryAction::ToSlot3) {
                Some(Some(2))
            } else if actions.just_pressed(&InventoryAction::ToSlot4) {
                Some(Some(3))
            } else if actions.just_pressed(&InventoryAction::ToSlot5) {
                Some(Some(4))
            } else {
                let (_, _, current_held_item) = arms
                    .iter()
                    .filter(|(_entity, arm, _held_item)| arm.parent == player_entity)
                    .next()
                    .unwrap();
                let (current_held_item, last_held_item) = (
                    current_held_item.held.map(|(a, b)| a).unwrap_or(0),
                    current_held_item.last_held_item,
                );
                let inventory_size = inventory.slots.len();
                if actions.just_pressed(&InventoryAction::PreviousWeapon) {
                    Some(Some((current_held_item - 1) % inventory_size))
                } else if actions.just_pressed(&InventoryAction::NextWeapon) {
                    Some(Some((current_held_item + 1) % inventory_size))
                } else if actions.just_pressed(&InventoryAction::LastUsedWeapon) {
                    Some(Some(last_held_item))
                } else if actions.just_pressed(&InventoryAction::Holster) {
                    Some(None)
                } else {
                    None
                }
            };
        if let Some(destination) = hotbar_destination {
            let arm_entity = arms
                .iter()
                .filter(|(_entity, arm, _held_item)| arm.parent == player_entity)
                .next()
                .unwrap()
                .0;
            change_slot.send(ChangeHeldInventoryItem {
                arm: arm_entity,
                slot: destination,
            });
        }
    }
}
