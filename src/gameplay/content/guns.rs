use bevy::prelude::Name;
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};

use crate::gameplay::{
    gunplay::{
        guns::Gun,
        servo::{FireMode, Servo},
        servo_components::ShootsBullet,
    },
    inventory::components::InventorySlotSize,
};

pub fn basic_gun<T: Into<String>>(name: T) -> ComponentTree {
    (
        Name::new(name.into()),
        Gun,
        Servo {
            firemode: FireMode::SemiAuto,
            // cooldown: todo!(),
            ..Default::default()
        },
    )
        .store()
}

pub fn pistol() -> ComponentTree {
    basic_gun("Pistol") + (InventorySlotSize::Small).store()
}

pub fn autorifle() -> ComponentTree {
    basic_gun("Auto Rifle") + (Servo::new(FireMode::FullAuto, 150)).store()
}
