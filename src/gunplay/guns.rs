use bevy::{
    audio::{AudioBundle, PlaybackSettings, Volume},
    core::Name,
    ecs::{
        component::Component,
        event::{EventReader, EventWriter},
        query::With,
        system::{Commands, Query, Res},
    },
    hierarchy::Children,
    math::Vec3,
    pbr::PbrBundle,
    reflect::Reflect,
    transform::components::{GlobalTransform, Transform},
};

use crate::util::camera_shake::TraumaEvent;
use crate::{
    asset_setup::{audio::PlaceholderAudio, primitives::PrimitiveResources},
    util::audio::SpawnAudioBlip,
};

use super::servo::ServoActivated;

#[derive(Component, Reflect)]
pub struct Gun;

#[derive(Component, Reflect)]
pub struct Barrel;

pub fn fire_guns(
    mut commands: Commands,
    mut events: EventReader<ServoActivated>,
    mut recoil: EventWriter<TraumaEvent>,
    guns: Query<&Children, With<Gun>>,
    barrels: Query<&GlobalTransform>,
    primitive_res: Res<PrimitiveResources>,
    placeholder_audio: Res<PlaceholderAudio>,
    mut audio_send: EventWriter<SpawnAudioBlip>,
) {
    for ServoActivated(entity) in events.read() {
        if let Ok(children) = guns.get(*entity) {
            let barrel = children.iter().next().unwrap();
            let (_, rot, loc) = barrels
                .get(*barrel)
                .unwrap()
                .to_scale_rotation_translation();
            let transform = Transform {
                translation: loc,
                rotation: rot,
                scale: Vec3::splat(0.2),
            };
            commands.spawn(Name::new("Bullet")).insert(PbrBundle {
                mesh: primitive_res.sphere.clone(),
                material: primitive_res.material.clone(),
                transform,
                ..Default::default()
            });
            recoil.send(0.1.into());

            audio_send.send(SpawnAudioBlip {
                handle: placeholder_audio.rifle1.clone(),
                location: loc,
                volume: 1.0,
                stick_to: Some(*barrel),
            });
        }
    }
}
