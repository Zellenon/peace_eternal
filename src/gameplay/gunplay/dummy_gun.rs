use bevy::{
    math::{Quat, Vec3},
    prelude::{
        Changed, Children, Commands, Component, Entity, Event, EventReader, EventWriter, Query,
        Res, Transform, Visibility, With, Without,
    },
    reflect::Reflect,
};
use bevy_composable::{app_impl::ComplexSpawnable, tree::ComponentTree};

use crate::{
    asset_setup::models::ModelResources,
    gameplay::{
        content::LinkedModel,
        inventory::{components::Inventory, swapping::ChangeHeldInventoryItem},
        levels_setup::IsPlayer,
    },
};

use super::{arms::Arm, guns::Gun};

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct DummyGun {
    representing: Option<Entity>,
}

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct Barrel;

/// If the dummy changes the object it represents, swap the model
pub fn swap_dummygun_model(
    mut commands: Commands,
    dummies: Query<(Entity, &DummyGun, &Children), Changed<DummyGun>>,
    guns: Query<(&Gun, &LinkedModel)>,
    mut barrels: Query<&mut Transform, With<Barrel>>,
    models: Res<ModelResources>,
) {
    for (dummy_entity, dummy, children) in dummies.iter() {
        if let (Some((gun, linked_model)), Some(mut dummy_entity)) = (
            dummy.representing.map(|w| guns.get(w).ok()).flatten(),
            commands.get_entity(dummy_entity),
        ) {
            dummy_entity.compose(linked_model.0(&*models));
            for maybe_barrel in children {
                if let Ok(mut barrel_transform) = barrels.get_mut(*maybe_barrel) {
                    barrel_transform.translation = gun.barrel_pos.0;
                    barrel_transform.rotation = gun.barrel_pos.1;
                }
            }
        }
    }
}

/// When the arm changes which item it's holding, change the dummy
pub fn swap_held_dummy_model(
    mut changes: EventReader<ChangeHeldInventoryItem>,
    arms: Query<(&Arm, &Children)>,
    inventories: Query<&Inventory, Without<Arm>>,
    mut dummy_guns: Query<&DummyGun>,
) {
    for change in changes.read() {
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
            .map(|((linked_model, barrel_position), children)| {
                children.iter().for_each(|child| {
                    dummy_guns.get(*child).ok().map(|dummy| {
                        model_changes.send(SwapDummyModel {
                            entity: dummy,
                            gunmesh: linked_model.0(&*models),
                            barrel_position: barrel_position.barrel_pos,
                        })
                    });
                })
            });
    }
}

pub fn hide_gun_on_empty_hand(
    mut changes: EventReader<ChangeHeldInventoryItem>,
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
