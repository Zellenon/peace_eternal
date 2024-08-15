use bevy::prelude::IntoSystemConfigs;
use bevy::{
    app::{Plugin, PostUpdate, PreUpdate, Startup, Update},
    transform::systems::{propagate_transforms, sync_simple_transforms},
};

use self::{
    camera_shake::{apply_trauma_events, restore, shake, Shake, ShakeSettings, TraumaEvent},
    primitives::{populate_primitive_resources, Primitive_Resources},
    smoothing::{smooth_movement, SmoothedTransform},
};

pub mod animating;
pub mod camera_shake;
pub mod primitives;
pub mod smoothing;

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<SmoothedTransform>();
        app.insert_resource(Primitive_Resources::default());

        app.add_systems(Startup, populate_primitive_resources);

        app.add_systems(Update, smooth_movement);

        app.register_type::<Shake>()
            .register_type::<ShakeSettings>()
            .add_systems(PreUpdate, restore)
            .add_systems(
                PostUpdate,
                shake
                    .before(propagate_transforms)
                    .before(sync_simple_transforms),
            );

        app.add_event::<TraumaEvent>()
            .add_systems(PostUpdate, apply_trauma_events.before(shake));
    }
}
