use bevy::app::{Plugin, Update};
use hud::hud_gui;

pub mod hud;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, hud_gui);
    }
}

