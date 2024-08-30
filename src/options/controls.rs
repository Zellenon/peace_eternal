use bevy::{ecs::system::Resource, reflect::Reflect};

#[derive(Resource, Reflect, Clone, Debug, PartialEq)]
pub struct ControlOptions {
    pub mouse_sensitivity: f32,
    pub invert_y: bool,
}

impl Default for ControlOptions {
    fn default() -> Self {
        ControlOptions {
            mouse_sensitivity: 0.20,
            invert_y: false,
        }
    }
}
