use bevy::{
    prelude::{Children, Component, EventReader, EventWriter, Query, Res, With},
    reflect::Reflect,
};
use bevy_composable::tree::ComponentTree;
use bevy_hanabi::EffectSpawner;

use crate::{
    asset_setup::audio::PlaceholderAudio,
    gameplay::content::projectiles::basic_bullet,
    graphics::{MuzzleFlashFX, SpawnAudioBlip, SpawnFlash, SpawnMuzzleFlare},
};

use super::{
    arms::Recoil, dummy_gun::Barrel, projectiles::FireProjectile, servo::DirectedServoActivated,
};

#[derive(Component, Clone)]
pub struct ShootsBullets {
    pub projectile: ComponentTree,
    pub accuracy: f32,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct HasMuzzleFlare {
    pub main_size: f32,
    pub petal_num: usize,
    pub petal_coef: f32,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct HasMuzzleFlash(f32);

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct HasGunSmoke;

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct HasRecoil;

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct HasActivationSound;

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct MultiActivation(pub usize);

pub fn gunshots_to_bullet_spawn(
    mut shots: EventReader<DirectedServoActivated>,
    mut fire_bullets: EventWriter<FireProjectile>,
    shooters: Query<&ShootsBullets>,
) {
    shots
        .read()
        .filter_map(|w| shooters.get(w.servo).ok().map(|shooter| (shooter, w)))
        .for_each(
            |(
                ShootsBullets,
                DirectedServoActivated {
                    servo: _,
                    barrel: _,
                    location,
                    rotation,
                },
            )| {
                fire_bullets.send(FireProjectile {
                    bullet: basic_bullet(),
                    location: *location,
                    rotation: *rotation,
                });
            },
        );
}

pub fn gunshots_to_muzzle_flash(
    mut shots: EventReader<DirectedServoActivated>,
    mut flash_spawns: EventWriter<SpawnFlash>,
    muzzle_flashes: Query<&HasMuzzleFlash>,
) {
    shots
        .read()
        .filter_map(|w| muzzle_flashes.get(w.servo).ok().map(|flash| (flash, w)))
        .for_each(
            |(
                HasMuzzleFlash(size),
                DirectedServoActivated {
                    servo: _,
                    barrel: _,
                    location,
                    rotation: _,
                },
            )| {
                flash_spawns.send(SpawnFlash {
                    location: *location,
                    size: *size,
                });
            },
        );
}

pub fn gunshots_to_muzzle_flare(
    mut shots: EventReader<DirectedServoActivated>,
    mut flare_spawns: EventWriter<SpawnMuzzleFlare>,
    muzzle_flares: Query<&HasMuzzleFlare>,
) {
    shots
        .read()
        .filter_map(|w| muzzle_flares.get(w.servo).ok().map(|flare| (flare, w)))
        .for_each(
            |(
                HasMuzzleFlare {
                    main_size,
                    petal_num,
                    petal_coef,
                },
                DirectedServoActivated {
                    servo: _,
                    barrel: _,
                    location,
                    rotation,
                },
            )| {
                flare_spawns.send(SpawnMuzzleFlare {
                    location: *location,
                    size: *main_size,
                    direction: *rotation,
                });
            },
        );
}

pub fn gunshots_spawn_muzzlefx(
    mut shots: EventReader<DirectedServoActivated>,
    barrels: Query<&Children, With<Barrel>>,
    mut muzzle_fx: Query<&mut EffectSpawner, With<MuzzleFlashFX>>,
) {
    for shot in shots.read() {
        if let Ok(barrel_fx) = barrels.get(shot.barrel) {
            barrel_fx.iter().for_each(|w| {
                let _ = muzzle_fx.get_mut(*w).map(|mut w2| w2.reset());
            });
        }
    }
}

pub fn do_activation_sounds(
    mut shots: EventReader<DirectedServoActivated>,
    mut blip_spawns: EventWriter<SpawnAudioBlip>,
    blippers: Query<&HasActivationSound>,
    placeholder_audio: Res<PlaceholderAudio>,
) {
    shots
        .read()
        .filter_map(|w| blippers.get(w.servo).ok().map(|blip| (blip, w)))
        .for_each(
            |(
                HasActivationSound,
                DirectedServoActivated {
                    servo: _,
                    barrel,
                    location,
                    rotation: _,
                },
            )| {
                blip_spawns.send(SpawnAudioBlip {
                    handle: placeholder_audio.rifle1.clone(),
                    location: *location,
                    volume: 1.0,
                    stick_to: Some(*barrel),
                });
            },
        );
}

pub fn gunshots_to_recoil(
    mut shots: EventReader<DirectedServoActivated>,
    mut recoils: EventWriter<Recoil>,
    recoilers: Query<&HasRecoil>,
) {
    shots
        .read()
        .filter_map(|w| recoilers.get(w.servo).ok().map(|recoil| (recoil, w)))
        .for_each(
            |(
                HasRecoil,
                DirectedServoActivated {
                    servo,
                    barrel,
                    location,
                    rotation: _,
                },
            )| {
                recoils.send(Recoil {
                    arm: *servo,
                    strength: 1.,
                });
            },
        );
}
