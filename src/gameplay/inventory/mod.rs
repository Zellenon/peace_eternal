use bevy::{
    app::{Plugin, Update},
    prelude::IntoSystemConfigs,
};
use components::{
    add_in_inventory, remove_old_in_inventory, FlavorText, InInventory, Inventory, InventorySlot,
    InventorySlotSettings, InventorySlotSize, Nickname,
};
use swapping::{
    add_held_by, do_change_held_item, remove_old_held_by, ChangeHeldItem, HeldBy,
    HoldingInventoryItem,
};

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
            .register_type::<Nickname>()
            .register_type::<FlavorText>()
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
