use super::{projectiles::basic_bullet, LinkedModel};
use crate::{
    asset_setup::{audio::PlaceholderAudio, models::ModelResources},
    gameplay::{
        gunplay::{
            guns::Gun,
            servo::{FireMode, Servo},
            servo_components::{
                HasActivationSound, HasGunSmoke, HasMuzzleFlare, HasMuzzleFlash, HasRecoil,
                MultiActivation, ShootsBullets,
            },
        },
        inventory::{components::InventorySlotSize, items::Nickname},
    },
};
use bevy::{
    asset::Handle,
    audio::AudioSource,
    math::{Quat, Vec3},
    prelude::Name,
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};

pub struct GunData {
    name: String,
    nickname: String,
    description: String,
    barrel_pos: (f32, f32, f32),
    barrel_angle: Quat,
    firemode: FireMode,
    cooldown: f32,
    size: InventorySlotSize,
    linked_model: LinkedModel,
    bullet_tree: ComponentTree,
    accuracy: f32,
    bullet_size: f32,
    bullet_speed: f32,
    muzzle_flare: Option<HasMuzzleFlare>,
    muzzle_flash: Option<f32>,
    recoil: Option<f32>,
    gunsmoke: bool,
    bullet_num: usize,
    sound: &'static (dyn Send + Sync + Fn(&PlaceholderAudio) -> Vec<Handle<AudioSource>>),
}

impl GunData {
    pub fn tree(&self) -> ComponentTree {
        name(self.name.clone())
            + (Gun {
                barrel_pos: (self.barrel_pos.into(), self.barrel_angle),
            })
            .store()
            + (
                Nickname(self.nickname.clone()),
                // Nickname(self.description.clone()),
                InventorySlotSize::Small,
                self.linked_model.clone(),
                ShootsBullets::new(
                    self.bullet_tree.clone(),
                    self.accuracy,
                    self.bullet_size,
                    self.bullet_speed,
                ),
                HasRecoil(1.),
                HasGunSmoke,
                HasActivationSound::new(PlaceholderAudio::rifles),
            )
                .store()
            + match &self.muzzle_flare {
                Some(flare) => flare.clone().store(),
                None => ().store(),
            }
            + match &self.muzzle_flash {
                Some(flash) => HasMuzzleFlash(*flash).store(),
                None => ().store(),
            }
            + match &self.recoil {
                Some(recoil) => HasRecoil(*recoil).store(),
                None => ().store(),
            }
            + match &self.recoil {
                Some(recoil) => HasRecoil(*recoil).store(),
                None => ().store(),
            }
            + match self.bullet_num {
                1 => ().store(),
                a => MultiActivation(a).store(),
            }
            + match self.gunsmoke {
                true => HasGunSmoke.store(),
                false => ().store(),
            }
    }
}

impl Default for GunData {
    fn default() -> Self {
        Self {
            name: "ERROR: UNFILLED NAME".into(),
            nickname: "ERROR: UNFILLED NICK".into(),
            description: "ERROR: UNFILLED DESC".into(),
            barrel_pos: (0., 0., 0.),
            barrel_angle: Quat::default(),
            firemode: FireMode::SemiAuto,
            cooldown: 0.2,
            size: InventorySlotSize::Small,
            linked_model: LinkedModel::new(ModelResources::basic_bullet),
            bullet_tree: basic_bullet(),
            accuracy: 0.95,
            bullet_size: 1.0,
            bullet_speed: 3.,
            muzzle_flare: None,
            muzzle_flash: None,
            recoil: None,
            gunsmoke: true,
            bullet_num: 1,
            sound: &PlaceholderAudio::jumps,
        }
    }
}

pub fn basic_gun<T: Into<String>, H: Into<Vec3>>(name: T, barrel_pos: (H, Quat)) -> ComponentTree {
    (
        Name::new(name.into()),
        Gun {
            barrel_pos: (barrel_pos.0.into(), barrel_pos.1),
        },
        Servo {
            firemode: FireMode::SemiAuto,
            // cooldown: todo!(),
            ..Default::default()
        },
    )
        .store()
}

pub fn pistol_1() -> GunData {
    GunData {
        name: "Entwell P1 Collector's Edition".into(),
        nickname: "EP1-C".into(),
        description: "A basic combat pistol distributed to interns.".into(),
        barrel_pos: (-0.01, 0.18, -0.4),
        cooldown: 0.3,
        linked_model: LinkedModel::new(ModelResources::pistol1),
        accuracy: 0.97,
        bullet_size: 0.7,
        bullet_speed: 3.5,
        muzzle_flare: Some(HasMuzzleFlare::new(1., 4, 0.3)),
        muzzle_flash: Some(0.3),
        recoil: Some(0.3),
        sound: &PlaceholderAudio::rifles,
        ..Default::default()
    }
}

pub fn pistol_2() -> GunData {
    GunData {
        name: "Febrell Professional Peashooter".into(),
        nickname: "FPP".into(),
        description: "A slightly older, heavier handgun.".into(),
        barrel_pos: (-0.01, 0.14, -0.4),
        cooldown: 0.5,
        linked_model: LinkedModel::new(ModelResources::pistol2),
        accuracy: 0.97,
        bullet_size: 1.,
        bullet_speed: 3.5,
        muzzle_flare: Some(HasMuzzleFlare::new(1., 2, 0.3)),
        muzzle_flash: Some(0.5),
        recoil: Some(0.6),
        sound: &PlaceholderAudio::rifles,
        ..Default::default()
    }
}

pub fn rifle_1() -> GunData {
    GunData {
        name: "Ruben & Decker 113".into(),
        nickname: "Ruben Deck13".into(),
        description:
            "A mass-produced Rub'n'Deck classic, lovingly nicknamed the 'Rubber Duckie' by adherents who kept up its use even after objectively better weaponry was developed."
                .into(),
        barrel_pos: (-0.01, 0.03, -0.4),
        cooldown: 0.08,
        linked_model: LinkedModel::new(ModelResources::rifle1),
        accuracy: 0.93,
        bullet_size: 0.5,
        bullet_speed: 3.5,
        muzzle_flare: Some(HasMuzzleFlare::new(1., 0, 0.0)),
        muzzle_flash: Some(0.1),
        recoil: Some(0.3),
        sound: &PlaceholderAudio::rifles,
        size: InventorySlotSize::Medium,
        ..Default::default()
    }
}

pub fn shotgun_combat() -> GunData {
    GunData {
        name: "CS15 - 'Capetown'".into(),
        nickname: "Capetown".into(),
        description: "".into(),
        barrel_pos: (-0.01, 0.2, -0.85),
        cooldown: 0.6,
        linked_model: LinkedModel::new(ModelResources::rifle2),
        accuracy: 0.82,
        bullet_size: 0.33,
        bullet_speed: 3.5,
        muzzle_flare: Some(HasMuzzleFlare::new(1., 6, 0.1)),
        muzzle_flash: Some(0.3),
        recoil: Some(0.8),
        sound: &PlaceholderAudio::jumps,
        size: InventorySlotSize::Medium,
        bullet_num: 12,
        ..Default::default()
    }
}

pub fn shotgun_big() -> GunData {
    GunData {
        name: "Thunderbuss".into(),
        nickname: "Thunderbuss".into(),
        description: "".into(),
        barrel_pos: (-0.01, 0.2, -0.4),
        cooldown: 0.9,
        linked_model: LinkedModel::new(ModelResources::shotgun_pump),
        accuracy: 0.60,
        bullet_size: 0.4,
        bullet_speed: 3.5,
        muzzle_flare: Some(HasMuzzleFlare::new(1.4, 4, 0.2)),
        muzzle_flash: Some(0.6),
        recoil: Some(1.2),
        sound: &PlaceholderAudio::rifles,
        size: InventorySlotSize::Medium,
        bullet_num: 22,
        ..Default::default()
    }
}

pub fn grenade_launcher() -> GunData {
    GunData {
        name: "Centinel 8 Light Infantry Grenade Launcher".into(),
        nickname: "C8LI GL".into(),
        description: "".into(),
        barrel_pos: (-0.01, 0.2, -0.4),
        accuracy: 0.93,
        cooldown: 0.6,
        linked_model: LinkedModel::new(ModelResources::grenade_launcher_1),
        bullet_size: 1.8,
        bullet_speed: 0.5,
        muzzle_flare: Some(HasMuzzleFlare::new(0.4, 0, 0.0)),
        muzzle_flash: Some(0.1),
        recoil: Some(0.5),
        sound: &PlaceholderAudio::rifles,
        size: InventorySlotSize::Medium,
        ..Default::default()
    }
}
