use bevy::app::{Plugin, Update};
use hud::{hotbar_ui, hud_ui};

pub mod hud;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (hotbar_ui, hud_ui));
    }
}
