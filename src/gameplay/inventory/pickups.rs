use avian3d::prelude::{Collider, LinearVelocity, RigidBody};
use bevy::{
    math::Vec3,
    pbr::PbrBundle,
    prelude::{Commands, Component, Entity, Event, EventReader, Query, Res, SpatialBundle, With},
    reflect::Reflect,
};
use bevy_composable::app_impl::{ComplexSpawnable, ComponentTreeable};

use crate::{
    asset_setup::{models::ModelResources, primitives::PrimitiveResources},
    gameplay::{content::LinkedModel, Gun},
};

#[derive(Debug, Reflect, Component, Clone, Copy)]
pub enum Pickup {
    Gun(Entity),
    Health(f32),
}

#[derive(Debug, Reflect, Event, Clone, Copy)]
pub struct SpawnPickup {
    drop: Pickup,
    pos: Vec3,
    vel: Vec3,
}

// TODO: Spawn things that aren't guns?
pub fn do_spawn_pickup(
    mut events: EventReader<SpawnPickup>,
    mut commands: Commands,
    guns: Query<&LinkedModel, With<Gun>>,
    gun_assets: Res<ModelResources>,
) {
    for SpawnPickup { drop, pos, vel } in events.read() {
        let model_fn = match drop {
            Pickup::Gun(entity) => guns.get(*entity).unwrap(),
            Pickup::Health(_) => todo!(),
        };
        commands.compose(
            (
                SpatialBundle {
                    transform: bevy::prelude::Transform::from_translation(*pos),
                    ..Default::default()
                },
                *drop,
                RigidBody::Static,
                LinearVelocity(*vel),
                Collider::cylinder(4.0, 0.1),
            )
                .store()
                + model_fn.0(&*gun_assets),
        );
    }
}
