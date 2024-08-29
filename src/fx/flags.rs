use bevy::{
    prelude::{Component, GlobalTransform, Parent, Query, With},
    reflect::Reflect,
};
use bevy_hanabi::EffectProperties;

#[derive(Debug, Reflect, Component)]
pub struct DirectedFX;

#[derive(Debug, Reflect, Component)]
pub struct MuzzleFlashFX;

pub(crate) fn update_fx_directions(
    parents: Query<&GlobalTransform>,
    mut fx: Query<(&Parent, &mut EffectProperties), With<DirectedFX>>,
) {
    for (parent, mut properties) in fx.iter_mut() {
        if let Ok(transform) = parents.get(parent.get()) {
            properties.set("direction", transform.back().as_vec3().into())
        }
    }
}
