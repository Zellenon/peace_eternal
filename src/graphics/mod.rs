pub use character_animating_systems::{animate_humanoids, AnimationState};
#[allow(unused_imports)]
pub use fx::{
    audio::SpawnAudioBlip, flags::MuzzleFlashFX, flash::SpawnFlash, muzzle_flare::SpawnMuzzleFlare,
    smokepuff::smoke_puff, sparks::basic_sparks, FXPlugin,
};

mod character_animating_systems;
mod fx;
