use bevy::{
    asset::Handle,
    ecs::system::Resource,
    prelude::{Image, Reflect},
};
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource, Reflect, Clone, Debug, PartialEq)]
pub struct ParticleTextures {
    #[asset(path = "particles/smoke.png")]
    pub smoke: Handle<Image>,
}

impl ParticleTextures {}
