use bevy::asset::Handle;
use bevy::audio::AudioSource;
use bevy::ecs::system::Resource;
use bevy::prelude::Reflect;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource, Reflect, Clone, Debug, PartialEq)]
pub struct PlaceholderAudio {
    #[asset(path = "audio/placeholder/bodyimpact.ogg")]
    pub body_impact: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/boing_01.ogg")]
    pub boing: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/death_01.ogg")]
    pub death1: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/death_02.ogg")]
    pub death2: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/footstep_01.ogg")]
    pub footstep1: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/footstep_02.ogg")]
    pub footstep2: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/footstep_03.ogg")]
    pub footstep3: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/footstep_04.ogg")]
    pub footstep4: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/get_armour.ogg")]
    pub get_armor: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/gun_load_01.ogg")]
    pub gun_load1: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/gun_load_02.ogg")]
    pub gun_load2: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/gun_machine.ogg")]
    pub machine_gun: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/gun_outofammo.ogg")]
    pub gun_outofammo: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/gun_rifle_01.ogg")]
    pub rifle1: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/gun_rifle_02.ogg")]
    pub rifle2: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/gun_shot.ogg")]
    pub gunshot: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/jump_01.ogg")]
    pub jump1: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/jump_02.ogg")]
    pub jump2: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/jump_03.ogg")]
    pub jump3: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/jump_04.ogg")]
    pub jump4: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/land.ogg")]
    pub land: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/pain_01.ogg")]
    pub pain1: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/pain_02.ogg")]
    pub pain2: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/pain_03.ogg")]
    pub pain3: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/powerup_01.ogg")]
    pub powerup1: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/powerup_02.ogg")]
    pub powerup2: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/powerup_03.ogg")]
    pub powerup3: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/powerup_04.ogg")]
    pub powerup4: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/punch_01.ogg")]
    pub punch1: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/punch_02.ogg")]
    pub punch2: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/splash_01.ogg")]
    pub splash1: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/splash_02.ogg")]
    pub splash2: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/splash_03.ogg")]
    pub splash3: Handle<AudioSource>,
    #[asset(path = "audio/placeholder/teleport.ogg")]
    pub teleport: Handle<AudioSource>,
}

impl PlaceholderAudio {
    pub fn death(&self) -> [Handle<AudioSource>; 2] {
        [self.death1.clone(), self.death2.clone()]
    }

    pub fn footsteps(&self) -> [Handle<AudioSource>; 4] {
        [
            self.footstep1.clone(),
            self.footstep2.clone(),
            self.footstep3.clone(),
            self.footstep4.clone(),
        ]
    }

    pub fn powerups(&self) -> [Handle<AudioSource>; 4] {
        [
            self.powerup1.clone(),
            self.powerup2.clone(),
            self.powerup3.clone(),
            self.powerup4.clone(),
        ]
    }

    pub fn punches(&self) -> [Handle<AudioSource>; 2] {
        [self.punch1.clone(), self.punch2.clone()]
    }

    pub fn splashes(&self) -> [Handle<AudioSource>; 3] {
        [
            self.splash1.clone(),
            self.splash2.clone(),
            self.splash3.clone(),
        ]
    }

    pub fn pain(&self) -> [Handle<AudioSource>; 3] {
        [self.pain1.clone(), self.pain2.clone(), self.pain3.clone()]
    }

    pub fn jumps(&self) -> [Handle<AudioSource>; 4] {
        [
            self.jump1.clone(),
            self.jump2.clone(),
            self.jump3.clone(),
            self.jump4.clone(),
        ]
    }

    pub fn rifles(&self) -> [Handle<AudioSource>; 2] {
        [self.rifle1.clone(), self.rifle2.clone()]
    }
}
