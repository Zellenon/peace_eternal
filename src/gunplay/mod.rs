use bevy::app::{Plugin, Update};
use bevy::prelude::IntoSystemConfigs;

use crate::character_control_systems::camera_controls::apply_mouse_camera_movement;

use self::arms::update_arm_position;

pub mod arms;

pub struct GunplayPlugin;

impl Plugin for GunplayPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            update_arm_position.after(apply_mouse_camera_movement),
        );
    }
}
