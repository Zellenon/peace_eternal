use bevy::app::{Plugin, Update};
use components::{Inventory, InventorySlot, InventorySlotSettings, InventorySlotSize};
use swapping::{do_change_held_item, ChangeHeldItem, HoldingInventoryItem};

pub mod components;
pub mod swapping;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ChangeHeldItem>();

        app.register_type::<Inventory>()
            .register_type::<InventorySlot>()
            .register_type::<InventorySlotSettings>()
            .register_type::<InventorySlotSize>()
            .register_type::<HoldingInventoryItem>();

        app.add_systems(Update, do_change_held_item);
    }
}
