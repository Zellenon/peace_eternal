use crate::util::{make_model, make_model_bundle};
use bevy::{asset::Handle, ecs::system::Resource, gltf::Gltf, prelude::Reflect, scene::Scene};
use bevy_asset_loader::asset_collection::AssetCollection;
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
        make_model(&self.gun_names, &self.gun_scene)
    }

    pub fn gun_bundle(&self) -> ComponentTree {
        make_model_bundle(&self.gun_names, &self.gun_scene)
    }
}
