use bevy::{
    prelude::{Component, Entity},
    reflect::Reflect,
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
