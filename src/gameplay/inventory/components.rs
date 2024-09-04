use bevy::{
    prelude::{Changed, Commands, Component, Entity, Query},
    reflect::{Reflect, TypeData},
};
use bevy_composable::app_impl::ComponentTreeable;
use bevy_composable::tree::ComponentTree;

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
pub struct Inventory {
    pub slots: Vec<InventorySlot>,
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct InventorySlot {
    pub settings: InventorySlotSettings,
    pub contents: Option<Entity>,
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct InventorySlotSettings {
    pub allowed_sizes: Vec<InventorySlotSize>,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InventorySlotSize {
    Large,
    Medium,
    Small,
}

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
pub struct Nickname(pub String);

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
pub struct FlavorText(pub String);

#[derive(Component, Reflect, Debug, Clone, PartialEq)]
pub struct InInventory(pub Entity);

impl InventorySlotSettings {
    pub fn new<T: Into<Vec<InventorySlotSize>>>(sizes: T) -> Self {
        Self {
            allowed_sizes: sizes.into(),
        }
    }
}

impl InventorySlot {
    pub fn new<T: Into<Vec<InventorySlotSize>>>(sizes: T) -> Self {
        Self {
            settings: InventorySlotSettings::new(sizes),
            contents: None,
        }
    }
}

use InventorySlotSize::Large;
use InventorySlotSize::Medium;
use InventorySlotSize::Small;

use super::swapping::HeldBy;
pub fn default_inventory() -> ComponentTree {
    Inventory {
        slots: vec![
            InventorySlot::new([Large, Medium, Small]),
            InventorySlot::new([Medium, Small]),
            InventorySlot::new([Small]),
        ],
    }
    .store()
}

pub fn add_in_inventory(
    inventories: Query<(Entity, &Inventory), Changed<Inventory>>,
    mut commands: Commands,
) {
    for (entity, inventory) in inventories.iter() {
        inventory
            .slots
            .iter()
            .filter_map(|w| w.contents)
            .for_each(|item| {
                if let Some(mut commands) = commands.get_entity(item) {
                    commands.insert(InInventory(entity));
                }
            });
    }
}

pub fn remove_old_in_inventory(
    inventories: Query<&Inventory, Changed<Inventory>>,
    mut commands: Commands,
    items: Query<(Entity, &HeldBy)>,
) {
    for (item, HeldBy(holder)) in items.iter() {
        if let Ok(inventory) = inventories.get(*holder) {
            if !(inventory
                .slots
                .iter()
                .filter_map(|w| w.contents)
                .map(|w| w)
                .collect::<Vec<_>>()
                .contains(&item))
            {
                commands.get_entity(item).unwrap().remove::<HeldBy>();
            }
        }
    }
}
