use bevy::prelude::Name;
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};

use crate::gameplay::gunplay::{
    guns::Gun,
    servo::{FireMode, Servo},
    servo_components::ShootsBullet,
};

pub fn basic_gun() -> ComponentTree {
    (
        Name::new("Gun"),
        Gun,
        Servo {
            firemode: FireMode::SemiAuto,
            // cooldown: todo!(),
            ..Default::default()
        },
        ShootsBullet,
    )
        .store()
}
