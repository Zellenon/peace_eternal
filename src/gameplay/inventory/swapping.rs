use bevy::{
    prelude::{Commands, Component, Entity, Event, EventReader, Query},
    reflect::Reflect,
};

use crate::gameplay::Arm;

use super::components::Inventory;

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
pub struct HoldingInventoryItem {
    pub held_slot: Option<usize>,
    pub last_held_item: usize,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
pub struct HeldBy(pub Entity);

impl HoldingInventoryItem {
    pub fn new() -> Self {
        Self {
            held_slot: None,
            last_held_item: 0,
        }
    }
}

impl Default for HoldingInventoryItem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Event, Reflect, Debug, Clone, PartialEq)]
pub struct ChangeHeldItem {
    pub arm: Entity,
    pub slot: Option<usize>,
}

pub fn do_change_held_item(
    mut events: EventReader<ChangeHeldItem>,
    mut item_holders: Query<&mut HoldingInventoryItem>,
) {
    for ChangeHeldItem { arm, slot } in events.read() {
        if let Ok(mut holder) = item_holders.get_mut(*arm) {
            holder.last_held_item = holder.held_slot.unwrap_or(holder.last_held_item);
            holder.held_slot = *slot;
        }
    }
}

pub fn add_held_by(
    inventories: Query<&Inventory>,
    arms: Query<&Arm>,
    mut events: EventReader<ChangeHeldItem>,
    mut commands: Commands,
) {
    for ChangeHeldItem {
        arm: arm_entity,
        slot,
    } in events.read()
    {
        if let Ok(arm) = arms.get(*arm_entity) {
            if let Ok(inventory) = inventories.get(arm.parent) {
                if let Some(item) = slot
                    .map(|held| inventory.slots.get(held).map(|w| w.contents))
                    .flatten()
                    .flatten()
                {
                    commands
                        .get_entity(item)
                        .unwrap()
                        .insert(HeldBy(*arm_entity));
                }
            }
        }
    }
}

pub fn remove_old_held_by(
    mut events: EventReader<ChangeHeldItem>,
    mut commands: Commands,
    items: Query<(Entity, &HeldBy)>,
) {
    for ChangeHeldItem { arm, slot } in events.read() {
        items.iter().filter(|w| w.0 == *arm).for_each(|(item, _)| {
            if let Some(mut commands) = commands.get_entity(item) {
                commands.remove::<HeldBy>();
            }
        });
    }
}
