use bevy_composable::app_impl::ComplexSpawnable;
use std::sync::Arc;

use bevy::{
    app::{Plugin, Update},
    prelude::{Added, Commands, Component, Entity, GlobalTransform, Query, Res, Visibility, With},
    reflect::Reflect,
};
use bevy_composable::tree::ComponentTree;

use crate::asset_setup::models::ModelResources;

pub mod guns;
pub mod projectiles;

#[derive(Component, Reflect, Clone)]
pub struct LinkedModel(pub Arc<dyn Sync + Send + Fn(&ModelResources) -> ComponentTree>);

impl LinkedModel {
    pub fn new<T: 'static + Sync + Send + Fn(&ModelResources) -> ComponentTree>(f: T) -> Self {
        Self(Arc::new(f))
    }
}

pub struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, load_linked_models);
    }
}

pub fn load_linked_models(
    mut commands: Commands,
    to_load: Query<
        (Entity, &LinkedModel),
        (With<GlobalTransform>, With<Visibility>, Added<LinkedModel>),
    >,
    models: Res<ModelResources>,
) {
    for (entity, linked_model) in to_load.iter() {
        if let Some(mut commands) = commands.get_entity(entity) {
            commands
                .remove::<LinkedModel>()
                .compose(linked_model.0(&*models));
        }
    }
}
