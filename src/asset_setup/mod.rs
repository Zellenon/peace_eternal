use audio::PlaceholderAudio;
use bevy::{app::Startup, prelude::Plugin};
use bevy_asset_loader::asset_collection::AssetCollectionApp;
use primitives::{populate_primitive_resources, PrimitiveResources};

pub mod audio;
pub mod primitives;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PrimitiveResources::default());
        app.init_collection::<PlaceholderAudio>();

        app.add_systems(Startup, populate_primitive_resources);
    }
}
