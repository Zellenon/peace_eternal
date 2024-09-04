use bevy::prelude::Name;
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};

use crate::{
    asset_setup::models::ModelResources,
    gameplay::{
        gunplay::{
            guns::Gun,
            servo::{FireMode, Servo},
            servo_components::{
                HasActivationSound, HasGunSmoke, HasMuzzleFlare, HasMuzzleFlash, HasRecoil,
                MultiActivation, ShootsBullets,
            },
        },
        inventory::components::{InventorySlotSize, Nickname},
    },
};

use super::{projectiles::basic_bullet, LinkedModel};

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

pub fn advanced_gun<T: Into<String>>(
    name: T,
    firemode: FireMode,
    cooldown: u64,
    size: InventorySlotSize,
) -> ComponentTree {
    basic_gun(name) + (Servo::new(firemode, cooldown), size).store()
}

pub fn pistol_1() -> ComponentTree {
    basic_gun("Entwell P15 Special")
        + (
            InventorySlotSize::Small,
            Nickname("EP-15S".to_owned()),
            LinkedModel::new(ModelResources::pistol1),
            ShootsBullets {
                projectile: basic_bullet(),
                accuracy: 0.97,
                scale: 0.5,
                force: 0.5,
            },
            HasMuzzleFlare {
                main_size: 1.,
                petal_num: 4,
                petal_coef: 0.1,
            },
            HasMuzzleFlash(1.),
            HasGunSmoke,
            HasRecoil(5.),
            HasActivationSound,
        )
            .store()
}

pub fn pistol_2() -> ComponentTree {
    basic_gun("Febrell Professional Peashooter")
        + (
            InventorySlotSize::Small,
            Nickname("FPP".to_owned()),
            LinkedModel::new(ModelResources::pistol2),
        )
            .store()
}

pub fn rifle_1() -> ComponentTree {
    basic_gun("Alt&Decker 113")
        + (
            Servo::new(FireMode::FullAuto, 150),
            Nickname("A&D-113".to_owned()),
            LinkedModel::new(ModelResources::rifle1),
        )
            .store()
}

pub fn shotgun_combat() -> ComponentTree {
    advanced_gun(
        "CS15 - 'Capetown'",
        FireMode::SemiAuto,
        500,
        InventorySlotSize::Medium,
    ) + (
        MultiActivation(10),
        Nickname("Capetown".to_owned()),
        LinkedModel::new(ModelResources::rifle2),
    )
        .store()
    // TODO: Low accuracy
    // TODO: Medium recoil
}

pub fn shotgun_big() -> ComponentTree {
    advanced_gun(
        "Thunderbuss",
        FireMode::SemiAuto,
        800,
        InventorySlotSize::Medium,
    ) + (
        MultiActivation(22),
        LinkedModel::new(ModelResources::shotgun_pump),
    )
        .store()
    // TODO: Piss accuracy
    // TODO: High recoil
}

pub fn grenade_launcher() -> ComponentTree {
    advanced_gun(
        "Centinel 8 Light Infantry Grenade Launcher",
        FireMode::SemiAuto,
        800,
        InventorySlotSize::Medium,
    ) + (
        Nickname("Stately GL".to_owned()),
        LinkedModel::new(ModelResources::grenade_launcher_1),
    )
        .store()
}
