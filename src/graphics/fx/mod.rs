use audio::{spawn_audio_blips, AudioBlip, SpawnAudioBlip};
use bevy::{
    app::{Plugin, Update},
    prelude::{IntoSystemConfigs, SystemSet},
    reflect::Reflect,
};
use bevy_hanabi::HanabiPlugin;

use crate::util::DestructionSet;
use flags::update_fx_directions;
use flash::{spawn_flash, SpawnFlash};
use muzzle_flare::{spawn_flare, SpawnMuzzleFlare};

pub(super) mod audio;
pub(super) mod flags;
pub(super) mod flash;
pub(super) mod muzzle_flare;
pub(super) mod smokepuff;
pub(super) mod sparks;

#[derive(Hash, Debug, Reflect, PartialEq, Eq, Clone, Copy, SystemSet)]
pub struct SpawnFXSet;

pub struct FXPlugin;

impl Plugin for FXPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SpawnFlash>()
            .add_event::<SpawnAudioBlip>()
            .add_event::<SpawnMuzzleFlare>();
        app.register_type::<SpawnFlash>()
            .register_type::<AudioBlip>();
        app.add_systems(
            Update,
            (spawn_audio_blips, spawn_flash, spawn_flare)
                .in_set(SpawnFXSet)
                .before(DestructionSet),
        );

        app.add_systems(Update, update_fx_directions);

        app.add_plugins(HanabiPlugin);
    }
}
