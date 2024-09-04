use bevy::{
    math::{Quat, Vec3},
    prelude::{
        Children, Commands, Component, Entity, Event, EventReader, EventWriter, Query, Res,
        Transform, Visibility, With, Without,
    },
    reflect::Reflect,
};
use bevy_composable::{app_impl::ComplexSpawnable, tree::ComponentTree};

use crate::{
    asset_setup::models::ModelResources,
    gameplay::{
        content::LinkedModel,
        inventory::{components::Inventory, swapping::ChangeHeldItem},
        levels_setup::IsPlayer,
    },
};

use super::{arms::Arm, guns::Gun};

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct DummyGun;

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct Barrel;

#[derive(Event, Clone)]
pub struct SwapDummyModel {
    pub entity: Entity,
    pub gunmesh: ComponentTree,
    pub barrel_position: (Vec3, Quat),
}

pub fn swap_dummygun_model(
    mut commands: Commands,
    mut swap_events: EventReader<SwapDummyModel>,
    guns: Query<&Children, With<DummyGun>>,
    mut barrels: Query<&mut Transform, With<Barrel>>,
) {
    for SwapDummyModel {
        entity,
        gunmesh,
        barrel_position,
    } in swap_events.read()
    {
        if let Some(mut dummy_entity) = commands.get_entity(*entity) {
            dummy_entity.compose(gunmesh.clone());

            for maybe_barrel in guns.get(*entity).unwrap() {
                if let Ok(mut barrel_transform) = barrels.get_mut(*maybe_barrel) {
                    barrel_transform.translation = barrel_position.0;
                    barrel_transform.rotation = barrel_position.1;
                }
            }
        }
    }
}

pub fn swap_held_dummy_model(
    mut model_changes: EventWriter<SwapDummyModel>,
    mut changes: EventReader<ChangeHeldItem>,
    arms: Query<(&Arm, &Children)>,
    inventories: Query<&Inventory, Without<Arm>>,
    real_guns: Query<&LinkedModel, With<Gun>>,
    dummy_guns: Query<Entity, With<DummyGun>>,
    models: Res<ModelResources>,
) {
    changes.read().for_each(|change| {
        arms.get(change.arm)
            .ok()
            .map(|(arm, children)| {
                inventories
                    .get(arm.parent)
                    .ok()
                    .map(|inventory| {
                        change
                            .slot
                            .map(|slot| inventory.slots.get(slot).map(|slot| slot.contents))
                    })
                    .flatten()
                    .flatten()
                    .flatten()
                    .map(|gun| real_guns.get(gun).ok())
                    .flatten()
                    .map(|w| (w, children))
            })
            .flatten()
            .map(|(linked_model, children)| {
                children.iter().for_each(|child| {
                    dummy_guns.get(*child).ok().map(|dummy| {
                        model_changes.send(SwapDummyModel {
                            entity: dummy,
                            gunmesh: linked_model.0(&*models),
                            barrel_position: (Vec3::new(-0.01, 0.2, -1.2), Quat::default()),
                        })
                    });
                })
            });
    });
    // for change in changes.read() {
    //     if let Ok((arm, children)) = arms.get(change.arm) {
    //         for child in children {
    //             if let Ok(dummy) = dummy_guns.get(*child) {
    //                 model_changes.send(SwapDummyModel {
    //                     entity: dummy,
    //                     gunmesh: models.gun_assets(),
    //                     barrel_position: (Vec3::new(-0.01, 0.2, -1.2), Quat::default()),
    //                 });
    //             }
    //         }
    //     }
    // }
}

pub fn hide_gun_on_empty_hand(
    mut changes: EventReader<ChangeHeldItem>,
    mut dummy_guns: Query<&mut Visibility, With<DummyGun>>,
    arms: Query<(&Arm, &Children), Without<IsPlayer>>,
    inventories: Query<&Inventory, Without<Arm>>,
) {
    for change in changes.read() {
        if let Ok((arm, arm_children)) = arms.get(change.arm) {
            arm_children.iter().for_each(|w| {
                if let Ok(mut dummy) = dummy_guns.get_mut(*w) {
                    match change.slot {
                        Some(slot) => {
                            let held_item = inventories
                                .get(arm.parent)
                                .ok()
                                .map(|inventory| inventory.slots.get(slot))
                                .flatten()
                                .map(|slot| slot.contents)
                                .flatten()
                                .is_some();
                            *dummy = match held_item {
                                true => Visibility::Visible,
                                false => Visibility::Hidden,
                            }
                        }
                        None => {
                            *dummy = Visibility::Hidden;
                        }
                    };
                }
            })
        }
    }
}
