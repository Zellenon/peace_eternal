use bevy::app::PostUpdate;
use bevy::prelude::IntoSystemConfigs;
use bevy::{
    app::{Plugin, Update},
    ecs::schedule::common_conditions::resource_changed,
};
use leafwing_input_manager::plugin::InputManagerPlugin;

use crate::util::camera_shake::shake;

use self::camera_controls::{
    apply_mouse_camera_movement, apply_scroll_zoom, mouse_should_control_camera,
    switch_first_third_person, update_fps_camera, update_tps_camera, CameraData, Facing,
};
use self::keyboard_receive::{CameraAction, PlayerAction, UiAction};
use self::mouse_grabbing::{
    grab_mouse_on_click, release_mouse_in_inventory, sync_mouse_grab, MouseGrabbed,
};

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

        app.register_type::<Facing>().register_type::<CameraData>();

        app.insert_resource(MouseGrabbed(false))
            .insert_resource(CameraData::default());

        app.add_systems(
            Update,
            sync_mouse_grab.run_if(resource_changed::<MouseGrabbed>),
        )
        .add_systems(
            Update,
            (
                (grab_mouse_on_click, release_mouse_in_inventory).before(sync_mouse_grab),
                (apply_mouse_camera_movement, apply_scroll_zoom)
                    .run_if(mouse_should_control_camera),
                (switch_first_third_person).after(apply_scroll_zoom),
            ),
        );

        app.add_systems(
            PostUpdate,
            (update_fps_camera, update_tps_camera)
                .before(shake)
                .before(bevy::transform::TransformSystem::TransformPropagate)
                .after(avian3d::prelude::PhysicsSet::Sync),
        );
    }
}
