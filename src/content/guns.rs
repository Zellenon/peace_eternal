use crate::gunplay::guns::Gun;
use crate::gunplay::servo::Servo;
use bevy::prelude::Name;
use bevy_composable::tree::ComponentTree;
use bevy_composable::tree::EntityCommandSet;
use bevy_composable::CT;

pub fn basic_gun(gun_assets: ComponentTree) -> ComponentTree {
    CT!(
        Name::new("Gun"),
        Gun,
        Servo {
            firemode: crate::gunplay::servo::FireMode::SemiAuto,
            // cooldown: todo!(),
            ..Default::default()
        }
    ) + gun_assets
}
