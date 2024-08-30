use bevy::prelude::IntoSystemConfigs;
use bevy::reflect::Reflect;
use bevy::{
    app::{Plugin, PostUpdate, PreUpdate, Update},
    transform::systems::{propagate_transforms, sync_simple_transforms},
};
use deathmarker::{
    delayed_death_markers, despawn_destroyed_entities, destroy_death_markers, end_lifespan,
    tick_lifespans, Deathmarker, Destroy, DestructionSet,
};

use self::{
    camera_shake::{apply_trauma_events, restore, shake, Shake, ShakeSettings, TraumaEvent},
    smoothing::{smooth_movement, SmoothedTransform},
};

pub mod animating;
pub mod camera_shake;
pub mod compose;
pub mod deathmarker;
pub mod smoothing;

#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<SmoothedTransform>()
            .register_type::<Deathmarker>();

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

        app.add_event::<Destroy>();
        app.add_systems(
            Update,
            (
                despawn_destroyed_entities,
                destroy_death_markers,
                delayed_death_markers,
            )
                .chain()
                .in_set(DestructionSet),
        )
        .add_systems(
            Update,
            (tick_lifespans, end_lifespan)
                .chain()
                .in_set(DestructionSet),
        );
    }
}
