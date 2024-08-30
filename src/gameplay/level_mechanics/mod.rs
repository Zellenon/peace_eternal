use bevy::prelude::*;

pub mod moving_platform;

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct LevelMechanicsPlugin;

impl Plugin for LevelMechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(moving_platform::MovingPlatformPlugin);
    }
}
