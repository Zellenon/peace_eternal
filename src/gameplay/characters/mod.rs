use avian3d::prelude::{Collider, RigidBody};
use bevy::{
    asset::Handle,
    audio::SpatialListener,
    gltf::Gltf,
    scene::{Scene, SceneBundle},
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree, wrappers::name};

use crate::util::GltfSceneHandler;

use super::{controls::camera_controls::Facing, levels_setup::IsPlayer};

pub fn character_base(
    character_scene: Handle<Scene>,
    character_scene_names: Handle<Gltf>,
) -> ComponentTree {
    name("Player")
        + (
            SceneBundle {
                scene: character_scene,
                ..Default::default()
            },
            GltfSceneHandler {
                names_from: character_scene_names,
            },
            RigidBody::Dynamic,
            Collider::capsule(0.5, 1.0),
            Facing::default(),
        )
            .store()
}

pub fn player_character_ingredients() -> ComponentTree {
    (IsPlayer, SpatialListener::new(2.0)).store() + name("PlayerCharacter")
}
