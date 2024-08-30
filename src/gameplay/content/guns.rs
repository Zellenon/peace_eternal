use bevy::prelude::Name;
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};

use crate::gameplay::gunplay::{
    guns::Gun,
    servo::{FireMode, Servo},
};

pub fn basic_gun(gun_assets: ComponentTree) -> ComponentTree {
    (
        Name::new("Gun"),
        Gun,
        Servo {
            firemode: FireMode::SemiAuto,
            // cooldown: todo!(),
            ..Default::default()
        },
    )
        .store()
        + gun_assets
}
