use crate::{asset_setup::primitives::PrimitiveResources, util::deathmarker::DelayedDeathmarker};
use bevy::{
    math::Vec3,
    prelude::{Commands, Event, EventReader, Name, PbrBundle, Res, Transform},
    reflect::Reflect,
};

#[derive(Event, Debug, Reflect)]
pub struct SpawnFlash {
    pub location: Vec3,
    pub size: f32,
}

pub(super) fn spawn_flash(
    mut events: EventReader<SpawnFlash>,
    mut commands: Commands,
    res: Res<PrimitiveResources>,
) {
    for SpawnFlash { location, size } in events.read() {
        commands.spawn((
            Name::new("Flash"),
            PbrBundle {
                mesh: res.sphere.clone(),
                material: res.bloom_material.clone(),
                transform: Transform::from_translation(*location).with_scale(Vec3::splat(*size)),
                ..Default::default()
            },
            DelayedDeathmarker,
        ));
    }
}
