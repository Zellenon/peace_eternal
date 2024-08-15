use bevy::app::{Plugin, Startup, Update};

use self::{
    primitives::{populate_primitive_resources, Primitive_Resources},
    smoothing::{smooth_movement, SmoothedTransform},
};

pub mod animating;
pub mod primitives;
pub mod smoothing;

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<SmoothedTransform>();
        app.insert_resource(Primitive_Resources::default());

        app.add_systems(Startup, populate_primitive_resources);

        app.add_systems(Update, smooth_movement);
    }
}
