use bevy::app::Plugin;

use self::controls::ControlOptions;

pub mod controls;

pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ControlOptions::default());
    }
}
