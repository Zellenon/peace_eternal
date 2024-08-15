use bevy::{
    core::Name,
    ecs::{
        component::Component,
        event::EventReader,
        query::With,
        system::{Commands, Query, Res},
    },
    hierarchy::Children,
    math::Vec3,
    pbr::PbrBundle,
    reflect::Reflect,
    transform::components::{GlobalTransform, Transform},
};

use crate::util::primitives::Primitive_Resources;

use super::arms::ServoActivated;

#[derive(Component, Reflect)]
pub struct Gun;

#[derive(Component, Reflect)]
pub struct Barrel;

pub fn fire_guns(
    mut commands: Commands,
    mut events: EventReader<ServoActivated>,
    guns: Query<&Children, With<Gun>>,
    barrels: Query<&GlobalTransform>,
    primitive_res: Res<Primitive_Resources>,
) {
    for ServoActivated(entity) in events.read() {
        if let Ok(children) = guns.get(*entity) {
            commands.spawn(Name::new("Bullet")).insert(PbrBundle {
                mesh: primitive_res.sphere.clone(),
                material: primitive_res.material.clone(),
                transform: {
                    let barrel = children.iter().next().unwrap();
                    let translation = barrels.get(*barrel).unwrap();
                    let (_, rot, loc) = translation.to_scale_rotation_translation();
                    Transform {
                        translation: loc,
                        rotation: rot,
                        scale: Vec3::splat(0.2),
                    }
                },
                ..Default::default()
            });
        }
    }
}
