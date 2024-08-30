use bevy::prelude::Name;
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};

use crate::gunplay::{guns::Gun, servo::Servo};

pub fn basic_gun(gun_assets: ComponentTree) -> ComponentTree {
    (
        Name::new("Gun"),
        Gun,
        Servo {
            firemode: crate::gunplay::servo::FireMode::SemiAuto,
            // cooldown: todo!(),
            ..Default::default()
        },
    )
        .store()
        + gun_assets
}
