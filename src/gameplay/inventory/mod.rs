use bevy::{
    app::{Plugin, Update},
    prelude::IntoSystemConfigs,
};
use components::{
    add_in_inventory, remove_old_in_inventory, InInventory, Inventory, InventorySlot,
    InventorySlotSettings, InventorySlotSize,
};
use items::{FlavorText, Nickname};
use pickups::{Pickup, SpawnPickup};
use swapping::{
    add_held_by, do_change_held_item, remove_old_held_by, ChangeHeldInventoryItem, HeldBy,
    HoldingInventoryItem,
};

pub mod components;
pub mod items;
pub mod pickups;
pub mod swapping;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ChangeHeldInventoryItem>();

        app.register_type::<Inventory>()
            .register_type::<InventorySlot>()
            .register_type::<InventorySlotSettings>()
            .register_type::<InventorySlotSize>()
            .register_type::<Nickname>()
            .register_type::<FlavorText>()
            .register_type::<Pickup>()
            .register_type::<SpawnPickup>()
            .register_type::<InInventory>()
            .register_type::<HeldBy>()
            .register_type::<HoldingInventoryItem>();

        app.add_systems(Update, (add_in_inventory, remove_old_in_inventory));
        app.add_systems(
            Update,
            (
                do_change_held_item,
                (remove_old_held_by, add_held_by).chain(),
            ),
        );
    }
}
