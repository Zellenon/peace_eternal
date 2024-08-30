use bevy::{app::Plugin, reflect::Reflect};

use self::controls::ControlOptions;

pub mod controls;

#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ControlOptions::default());
    }
}
