use bevy::prelude::Reflect;
use bevy::{
    ecs::{
        component::Component,
        query::With,
        system::{Query, Res},
    },
    math::{Quat, Vec2, Vec3},
    render::camera::Camera,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};
use bevy_tnua::math::{float_consts, AdjustPrecision, AsF32, Quaternion};
use bevy_tnua_physics_integration_layer::math::{Float, Vector3};
use leafwing_input_manager::action_state::ActionState;

use crate::options::controls::ControlOptions;

use super::keyboard_receive::CameraAction;

#[derive(Component, Reflect)]
pub struct FollowingCamera {
    pub forward: Vector3,
    pub pitch_angle: Float,
    pub distance: Float,
    pub shoulder_shift: Float,
}

impl Default for FollowingCamera {
    fn default() -> Self {
        Self {
            forward: Vector3::NEG_Z,
            pitch_angle: 0.0,
            distance: 5.,
            shoulder_shift: 0.2,
        }
    }
}

impl FollowingCamera {
    fn is_first_person(&self) -> bool {
        self.distance < 2.
    }
}

pub fn is_in_first_person(camera: Query<&FollowingCamera>) -> bool {
    camera.get_single().unwrap().is_first_person()
}

pub fn mouse_should_control_camera(
    player_camera_movement: Query<&Window, With<PrimaryWindow>>,
) -> bool {
    player_camera_movement
        .get_single()
        .map_or(false, |w| !w.cursor.visible)
}

pub fn apply_mouse_camera_movement(
    mut player_character_query: Query<(&mut FollowingCamera, &ActionState<CameraAction>)>,
    control_options: Res<ControlOptions>,
) {
    if let Ok((mut forward_from_camera, action_state)) = player_character_query.get_single_mut() {
        let total_movement =
            action_state.axis_pair(&CameraAction::Orbit) * control_options.mouse_sensitivity * {
                if control_options.invert_y {
                    Vec2::new(1., -1.)
                } else {
                    Vec2::ONE
                }
            };
        let x_shift = Quaternion::from_rotation_y(-0.01 * total_movement.x.adjust_precision());
        forward_from_camera.forward = x_shift.mul_vec3(forward_from_camera.forward);

        let pitch = 0.005 * total_movement.y.adjust_precision();
        forward_from_camera.pitch_angle = (forward_from_camera.pitch_angle + pitch)
            .clamp(-float_consts::FRAC_PI_2, float_consts::FRAC_PI_2);
    }
}

pub fn apply_scroll_zoom(
    mut player_character_query: Query<(&mut FollowingCamera, &ActionState<CameraAction>)>,
) {
    if let Ok((mut forward_from_camera, action_state)) = player_character_query.get_single_mut() {
        let zoom = action_state.clamped_value(&CameraAction::Zoom) * 0.75;
        forward_from_camera.distance = (forward_from_camera.distance - zoom).clamp(0., 13.);
    }
}

pub(crate) fn camera_follow_player(
    player_character_query: Query<(&GlobalTransform, &FollowingCamera)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if let Ok((player_transform, forward_from_camera)) = player_character_query.get_single() {
        if forward_from_camera.is_first_person() {
            for mut camera in camera_query.iter_mut() {
                camera.translation = player_transform.translation() + 0.7 * Vec3::Y;
                camera.look_to(forward_from_camera.forward.f32(), Vec3::Y);
                let pitch_axis = camera.left();
                camera.rotate_around(
                    player_transform.translation() + -0.5 * Vec3::Y,
                    Quat::from_axis_angle(*pitch_axis, forward_from_camera.pitch_angle.f32()),
                );
            }
        } else {
            for mut camera in camera_query.iter_mut() {
                let distance_from_player =
                    -1.0 * forward_from_camera.distance * forward_from_camera.forward.f32();
                let shoulder_shift = forward_from_camera.shoulder_shift
                    * forward_from_camera.distance
                    * forward_from_camera.forward.cross(Vec3::Y).f32();
                camera.translation = player_transform.translation()
                    + distance_from_player
                    + shoulder_shift
                    + 0.7 * Vec3::Y;
                camera.look_to(forward_from_camera.forward.f32(), Vec3::Y);
                let pitch_axis = camera.left();
                camera.rotate_around(
                    player_transform.translation() + -0.5 * Vec3::Y,
                    Quat::from_axis_angle(*pitch_axis, forward_from_camera.pitch_angle.f32()),
                );
            }
        }
    } else {
        return;
    };
}
