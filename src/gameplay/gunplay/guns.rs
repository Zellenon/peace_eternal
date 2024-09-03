use bevy::{
    ecs::{
        component::Component,
        event::{EventReader, EventWriter},
        query::With,
        system::{Commands, Query, Res},
    },
    hierarchy::Children,
    math::{Quat, Vec3},
    prelude::{Entity, Event, Parent, Without},
    reflect::Reflect,
    transform::components::GlobalTransform,
};
use bevy_composable::app_impl::ComplexSpawnable;
use bevy_hanabi::EffectSpawner;

use crate::{
    asset_setup::{audio::PlaceholderAudio, primitives::PrimitiveResources},
    gameplay::content::projectiles::basic_bullet,
    graphics::{MuzzleFlashFX, SpawnAudioBlip, SpawnFlash, SpawnMuzzleFlare},
    util::{instant_force, with_translation},
};

use super::{
    arms::Recoil,
    dummy_gun::Barrel,
    projectiles::FireProjectile,
    servo::{DirectedServoActivated, ServoActivated},
};

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct Gun;

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
pub struct FireGun {
    pub entity: Entity,
    pub position: Vec3,
    pub orientation: Quat,
}

// pub fn fire_guns(
//     mut servo_activations: EventReader<DirectedServoActivated>,
//     mut recoils: EventWriter<Recoil>,
//     mut audio_send: EventWriter<SpawnAudioBlip>,
//     mut flash_send: EventWriter<SpawnFlash>,
//     mut flare_send: EventWriter<SpawnMuzzleFlare>,
//     mut gunshots: EventWriter<FireGun>,
//     guns: Query<(&Children, &Parent), (With<Gun>, Without<Barrel>)>,
//     mut muzzle_flash_particles: Query<&mut EffectSpawner, With<MuzzleFlashFX>>,
//     barrels: Query<(&GlobalTransform, &Children), Without<Gun>>,
//     placeholder_audio: Res<PlaceholderAudio>,
// ) {
//     for ServoActivated(entity) in servo_activations.read() {
//         if let Ok((children, parent)) = guns.get(*entity) {
//             let (barrel, barrel_fx) = barrels.get(*children.iter().next().unwrap()).unwrap();
//             let (_, rot, loc) = barrel.to_scale_rotation_translation();

//             audio_send.send(SpawnAudioBlip {
//                 handle: placeholder_audio.rifle1.clone(),
//                 location: loc,
//                 volume: 1.0,
//                 stick_to: Some(parent.get()),
//             });

//             flash_send.send(SpawnFlash {
//                 location: loc,
//                 size: 0.3,
//             });

//             flare_send.send(SpawnMuzzleFlare {
//                 location: loc,
//                 size: 0.5,
//                 direction: rot,
//             });

//             recoils.send(Recoil {
//                 arm: parent.get(),
//                 strength: 1.,
//             });

//             barrel_fx.iter().for_each(|w| {
//                 let _ = muzzle_flash_particles.get_mut(*w).map(|mut w2| w2.reset());
//             });

//             gunshots.send(FireGun {
//                 entity: *entity,
//                 position: loc,
//                 orientation: rot,
//             });
//         }
//     }
// }
