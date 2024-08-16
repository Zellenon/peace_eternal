use bevy::{
    asset::Handle,
    audio::{AudioBundle, AudioSource, PlaybackSettings, Volume},
    core::Name,
    math::Vec3,
    prelude::{
        BuildChildren, Commands, Component, Entity, Event, EventReader, SpatialBundle, Transform,
    },
    reflect::Reflect,
};

#[derive(Component, Debug, Reflect)]
pub struct AudioBlip;

#[derive(Event, Reflect, Debug)]
pub struct SpawnAudioBlip {
    pub handle: Handle<AudioSource>,
    pub location: Vec3,
    pub volume: f32,
    pub stick_to: Option<Entity>,
}

pub(crate) fn spawn_audio_blips(mut commands: Commands, mut events: EventReader<SpawnAudioBlip>) {
    for SpawnAudioBlip {
        handle,
        location,
        volume,
        stick_to,
    } in events.read()
    {
        let audio_bundle = (
            AudioBlip,
            Name::new("Audio Blip"),
            AudioBundle {
                source: handle.clone(),
                settings: PlaybackSettings::DESPAWN
                    .with_volume(Volume::new(*volume))
                    .with_spatial(true),
            },
        );
        if let Some(stick_to) = stick_to {
            if let Some(mut parent) = commands.get_entity(*stick_to) {
                parent.with_children(|w| {
                    w.spawn((
                        audio_bundle,
                        SpatialBundle {
                            visibility: bevy::prelude::Visibility::Hidden,
                            ..Default::default()
                        },
                    ));
                });
            }
        } else {
            commands.spawn((
                audio_bundle,
                SpatialBundle {
                    visibility: bevy::prelude::Visibility::Hidden,
                    transform: Transform::from_translation(*location),
                    ..Default::default()
                },
            ));
        }
    }
}
