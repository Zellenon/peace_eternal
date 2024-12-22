use bevy::{
    prelude::{Commands, Component, Entity, Event, EventReader, Query},
    reflect::Reflect,
};

use crate::gameplay::Arm;

use super::components::Inventory;

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub struct HoldingInventoryItem {
    pub held: Option<(usize, Entity)>,
    pub last_held_item: usize,
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub struct HoldingItem(pub Entity);

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
pub struct HeldBy(pub Entity);

impl HoldingInventoryItem {
    pub fn new() -> Self {
        Self {
            held: None,
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
pub struct ChangeHeldInventoryItem {
    pub arm: Entity,
    pub slot: Option<usize>,
}

pub fn do_change_held_item(
    mut events: EventReader<ChangeHeldInventoryItem>,
    mut item_holders: Query<(&mut HoldingInventoryItem, &Arm)>,
    inventories: Query<&Inventory>,
) {
    for ChangeHeldInventoryItem { arm, slot } in events.read() {
        if let Ok((mut holder, arm)) = item_holders.get_mut(*arm) {
            if let Ok(inventory) = inventories.get(arm.parent) {
                holder.last_held_item =
                    holder.held.map(|(a, b)| a).unwrap_or(holder.last_held_item);
                holder.held = (*slot, holder.held.1.map(|w| inventory.slots.get(w)));
            }
        }
    }
}

pub fn add_held_by(
    inventories: Query<&Inventory>,
    arms: Query<&Arm>,
    mut events: EventReader<ChangeHeldInventoryItem>,
    mut commands: Commands,
) {
    for ChangeHeldInventoryItem {
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
    mut events: EventReader<ChangeHeldInventoryItem>,
    mut commands: Commands,
    items: Query<(Entity, &HeldBy)>,
) {
    for ChangeHeldInventoryItem { arm, slot } in events.read() {
        items.iter().filter(|w| w.0 == *arm).for_each(|(item, _)| {
            if let Some(mut commands) = commands.get_entity(item) {
                commands.remove::<HeldBy>();
            }
        });
    }
}
