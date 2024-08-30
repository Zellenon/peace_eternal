use crate::util::GltfSceneHandler;
use bevy::{
    asset::Handle,
    ecs::system::Resource,
    gltf::Gltf,
    prelude::{Reflect, SceneBundle},
    scene::Scene,
};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_composable::app_impl::ComponentTreeable;
use bevy_composable::tree::ComponentTree;

#[derive(AssetCollection, Resource, Reflect, Clone, Debug, PartialEq)]
pub struct ModelResources {
    #[asset(path = "models/gun.glb")]
    pub gun_names: Handle<Gltf>,
    #[asset(path = "models/gun.glb#Scene0")]
    pub gun_scene: Handle<Scene>,
    #[asset(path = "models/muzzleflare.glb")]
    pub flare_names: Handle<Gltf>,
    #[asset(path = "models/muzzleflare.glb#Scene0")]
    pub flare_scene: Handle<Scene>,
}

impl ModelResources {
    pub fn gun_assets(&self) -> ComponentTree {
        let gun_scene = self.gun_scene.clone();
        let gun_names = self.gun_names.clone();
        (
            SceneBundle {
                scene: gun_scene.clone(),
                ..Default::default()
            },
            GltfSceneHandler {
                names_from: gun_names.clone(),
            },
        )
            .store()
    }
}
