use bevy::{
    ecs::{
        component::Component,
        event::{EventReader, EventWriter},
        query::With,
        system::{Commands, Query, Res},
    },
    hierarchy::Children,
    math::Vec3,
    prelude::Parent,
    reflect::Reflect,
    transform::components::{GlobalTransform, Transform},
};
use bevy_composable::app_impl::ComplexSpawnable;

use crate::{
    asset_setup::{audio::PlaceholderAudio, primitives::PrimitiveResources},
    content::projectiles::basic_bullet,
    fx::{audio::SpawnAudioBlip, flash::SpawnFlash},
    util::compose::{instant_force, with_translation},
};

use super::{arms::Recoil, servo::ServoActivated};

#[derive(Component, Reflect)]
pub struct Gun;

#[derive(Component, Reflect)]
pub struct Barrel;

pub fn fire_guns(
    mut commands: Commands,
    mut servo_activations: EventReader<ServoActivated>,
    mut recoils: EventWriter<Recoil>,
    mut audio_send: EventWriter<SpawnAudioBlip>,
    mut flash_send: EventWriter<SpawnFlash>,
    guns: Query<(&Children, &Parent), With<Gun>>,
    barrels: Query<&GlobalTransform>,
    placeholder_audio: Res<PlaceholderAudio>,
    primitives: Res<PrimitiveResources>,
) {
    for ServoActivated(entity) in servo_activations.read() {
        if let Ok((children, parent)) = guns.get(*entity) {
            let barrel = children.iter().next().unwrap();
            let (_, rot, loc) = barrels
                .get(*barrel)
                .unwrap()
                .to_scale_rotation_translation();
            let transform = Transform {
                translation: loc,
                rotation: rot,
                scale: Vec3::splat(0.1),
            };
            // commands.spawn((
            //     Name::new("Bullet"),
            //     PbrBundle {
            //         mesh: primitive_res.sphere.clone(),
            //         material: primitive_res.bloom_material.clone(),
            //         transform,
            //         ..Default::default()
            //     },
            //     DelayedDeathmarker,
            // ));
            commands.spawn_complex(
                basic_bullet(&primitives.sphere, &primitives.material)
                    + with_translation(loc, rot, 0.1)
                    + instant_force(rot, 0.08),
            );

            audio_send.send(SpawnAudioBlip {
                handle: placeholder_audio.rifle1.clone(),
                location: loc,
                volume: 1.0,
                stick_to: Some(*barrel),
            });

            flash_send.send(SpawnFlash {
                location: loc,
                size: 0.3,
            });

            recoils.send(Recoil {
                arm: parent.get(),
                strength: 1.,
            });
        }
    }
}
