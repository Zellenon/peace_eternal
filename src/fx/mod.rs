use audio::{spawn_audio_blips, AudioBlip, SpawnAudioBlip};
use bevy::{
    app::{Plugin, Update},
    prelude::{IntoSystemConfigs, SystemSet},
    reflect::Reflect,
};
use flash::{spawn_flash, SpawnFlash};

use crate::util::deathmarker::DestructionSet;

pub mod audio;
pub mod flash;

#[derive(Hash, Debug, Reflect, PartialEq, Eq, Clone, Copy, SystemSet)]
pub struct SpawnFXSet;

pub struct FXPlugin;

impl Plugin for FXPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SpawnFlash>().add_event::<SpawnAudioBlip>();
        app.register_type::<SpawnFlash>()
            .register_type::<AudioBlip>();
        app.add_systems(
            Update,
            (spawn_audio_blips, spawn_flash)
                .in_set(SpawnFXSet)
                .before(DestructionSet),
        );
    }
}
