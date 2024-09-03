use bevy::{
    prelude::{Component, Entity, Event, EventReader, Query},
    reflect::Reflect,
};

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
pub struct HoldingInventoryItem {
    pub held_slot: Option<usize>,
    pub last_held_item: usize,
}

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
