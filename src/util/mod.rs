use bevy::app::{Plugin, Update};

use self::smoothing::{smooth_movement, SmoothedTransform};

pub mod animating;
pub mod smoothing;

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<SmoothedTransform>();

        app.add_systems(Update, smooth_movement);
    }
}
