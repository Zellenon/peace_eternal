use crate::{
    asset_setup::{models::ModelResources, primitives::PrimitiveResources},
    util::{animating::GltfSceneHandler, deathmarker::DelayedDeathmarker},
};
use bevy::{
    math::{Quat, Vec3},
    prelude::{Commands, Event, EventReader, Name, Res, Transform},
    reflect::Reflect,
    scene::SceneBundle,
};

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
pub struct SpawnMuzzleFlare {
    pub location: Vec3,
    pub size: f32,
    pub direction: Quat,
}

pub(super) fn spawn_flare(
    mut events: EventReader<SpawnMuzzleFlare>,
    mut commands: Commands,
    models: Res<ModelResources>,
    primitives: Res<PrimitiveResources>,
) {
    for SpawnMuzzleFlare {
        location,
        size,
        direction,
    } in events.read()
    {
        commands
            .spawn((
                Name::new("Flare"),
                SceneBundle {
                    scene: models.flare_scene.clone(),
                    transform: Transform {
                        translation: *location,
                        rotation: *direction,
                        scale: Vec3::splat(*size),
                    },
                    ..Default::default()
                },
                GltfSceneHandler {
                    names_from: models.flare_names.clone(),
                },
                DelayedDeathmarker,
            ))
            .insert(primitives.bloom_material.clone());
    }
}
