use bevy::{
    app::{Plugin, PostUpdate, PreUpdate, Update},
    math::{Quat, Vec3},
    prelude::{IntoSystemConfigs, Transform},
    reflect::Reflect,
    transform::systems::{propagate_transforms, sync_simple_transforms},
};

use camera_shake::{apply_trauma_events, restore_from_shake, shake, ShakeSettings};
use deathmarker::{
    delayed_death_markers, despawn_destroyed_entities, destroy_death_markers, end_lifespan,
    kill_small_enough, shrink_death, tick_lifespans, Destroy,
};

pub use animating::{
    animation_patcher_system, make_model, make_model_bundle, AnimationsHandler, GltfSceneHandler,
};
pub use camera_shake::{Shake, TraumaEvent};
#[allow(unused_imports)]
pub use compose::{instant_force, with_transform};
pub use deathmarker::{Deathmarker, DestructionSet, Lifespan, ShrinkDeath};
use smoothing::SmoothingPlugin;
pub use smoothing::{Smoothable, Smoothed};

mod ab;
mod animating;
mod camera_shake;
mod compose;
mod deathmarker;
mod smoothing;

#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<Deathmarker>();

        app.add_plugins((
            SmoothingPlugin::<Transform, Vec3, "translation">::default(),
            SmoothingPlugin::<Transform, Quat, "rotation">::default(),
        ));

        // app.register_type::<Shake>()
        //     .register_type::<ShakeSettings>()
        //     .add_systems(PreUpdate, restore_from_shake)
        //     .add_systems(
        //         PostUpdate,
        //         shake
        //             .before(propagate_transforms)
        //             .before(sync_simple_transforms),
        //     );

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
        )
        .add_systems(Update, (shrink_death, kill_small_enough));
    }
}
