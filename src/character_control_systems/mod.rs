use bevy::app::PostUpdate;
use bevy::prelude::IntoSystemConfigs;
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::common_conditions::resource_changed,
};
use leafwing_input_manager::plugin::InputManagerPlugin;

use self::camera_controls::{
    apply_mouse_camera_movement, camera_follow_player, mouse_should_control_camera,
};
use self::keyboard_receive::{CameraAction, PlayerAction, UiAction};
use self::mouse_grabbing::{grab_mouse_on_click, sync_mouse_grab, MouseGrabbed};

pub mod camera_controls;
pub mod info_dumping_systems;
pub mod keyboard_receive;
pub mod mouse_grabbing;
pub mod platformer_control_systems;

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<PlayerAction>()
            .register_type::<CameraAction>()
            .register_type::<UiAction>()
            .add_plugins((
                InputManagerPlugin::<PlayerAction>::default(),
                InputManagerPlugin::<CameraAction>::default(),
                InputManagerPlugin::<UiAction>::default(),
            ));

        app.insert_resource(MouseGrabbed(false));
        app.add_systems(
            Update,
            sync_mouse_grab.run_if(resource_changed::<MouseGrabbed>),
        )
        .add_systems(
            Update,
            (
                grab_mouse_on_click.before(sync_mouse_grab),
                apply_mouse_camera_movement
                    .before(camera_follow_player)
                    .run_if(mouse_should_control_camera),
            ),
        );

        app.add_systems(
            PostUpdate,
            camera_follow_player
                .before(bevy::transform::TransformSystem::TransformPropagate)
                .after(avian3d::prelude::PhysicsSet::Sync),
        );
    }
}
