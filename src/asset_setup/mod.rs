use audio::PlaceholderAudio;
use bevy::{app::Startup, prelude::Plugin};
use bevy_asset_loader::asset_collection::AssetCollectionApp;
use models::ModelResources;
use particles::ParticleTextures;
use primitives::{populate_primitive_resources, PrimitiveResources};

pub mod audio;
pub mod models;
pub mod particles;
pub mod primitives;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PrimitiveResources::default());
        app.init_collection::<PlaceholderAudio>()
            .init_collection::<ParticleTextures>()
            .init_collection::<ModelResources>();

        app.add_systems(Startup, populate_primitive_resources);
    }
}
