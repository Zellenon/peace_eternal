use bevy::{
    ecs::{
        component::Component,
        query::With,
        system::{Query, Res, ResMut},
    },
    math::{Quat, Vec2, Vec3},
    prelude::{Reflect, Resource, Visibility},
    render::camera::Camera,
    transform::components::GlobalTransform,
    window::{PrimaryWindow, Window},
};
use bevy_tnua::math::{float_consts, AdjustPrecision, AsF32, Quaternion};
use bevy_tnua_physics_integration_layer::math::{Float, Vector3};
use leafwing_input_manager::action_state::ActionState;

use crate::{levels_setup::IsPlayer, options::controls::ControlOptions, util::SmoothedTransform};

use super::keyboard_receive::CameraAction;

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct Facing {
    pub forward: Vector3,
    pub pitch_angle: Float,
}

impl Default for Facing {
    fn default() -> Self {
        Self {
            forward: Vector3::NEG_Z,
            pitch_angle: 0.0,
        }
    }
}

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct FPSCamera;

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct TPSCamera;

#[derive(Resource, Reflect, Clone, Debug, PartialEq)]
pub struct CameraData {
    pub distance: f32,
    pub shoulder_shift: f32,
}

impl CameraData {
    fn is_first_person(&self) -> bool {
        self.distance < 2.
    }
}

impl Default for CameraData {
    fn default() -> Self {
        Self {
            distance: 5.,
            shoulder_shift: 0.2,
        }
    }
}

pub fn is_in_first_person(camera: Res<CameraData>) -> bool {
    camera.is_first_person()
}

pub fn mouse_should_control_camera(
    player_camera_movement: Query<&Window, With<PrimaryWindow>>,
) -> bool {
    player_camera_movement
        .get_single()
        .map_or(false, |w| !w.cursor.visible)
}

pub fn apply_mouse_camera_movement(
    mut player_character_query: Query<(&mut Facing, &ActionState<CameraAction>)>,
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
    player_character_query: Query<&ActionState<CameraAction>>,
    mut camera_data: ResMut<CameraData>,
) {
    if let Ok(action_state) = player_character_query.get_single() {
        let zoom = action_state.clamped_value(&CameraAction::Zoom) * 0.75;
        camera_data.distance = (camera_data.distance - zoom).clamp(1., 13.);
    }
}

pub(crate) fn update_fps_camera(
    player_character_query: Query<(&GlobalTransform, &Facing), With<IsPlayer>>,
    mut camera_query: Query<&mut SmoothedTransform, With<FPSCamera>>,
) {
    if let Ok((player_transform, facing)) = player_character_query.get_single() {
        for mut camera in camera_query.iter_mut() {
            camera.goal.translation = player_transform.translation() + 0.7 * Vec3::Y;
            camera.goal.look_to(facing.forward.f32(), Vec3::Y);
            let pitch_axis = camera.goal.left();
            camera.goal.rotate_around(
                player_transform.translation() + -0.5 * Vec3::Y,
                Quat::from_axis_angle(*pitch_axis, facing.pitch_angle.f32()),
            );
        }
    }
}

pub(crate) fn update_tps_camera(
    player_character_query: Query<(&GlobalTransform, &Facing), With<IsPlayer>>,
    camera_data: Res<CameraData>,
    mut camera_query: Query<&mut SmoothedTransform, With<TPSCamera>>,
) {
    if let Ok((player_transform, facing)) = player_character_query.get_single() {
        if camera_data.is_first_person() {
        } else {
            for mut camera in camera_query.iter_mut() {
                let distance_from_player = -1.0 * camera_data.distance * facing.forward.f32();
                let shoulder_shift = camera_data.shoulder_shift
                    * camera_data.distance
                    * facing.forward.cross(Vec3::Y).f32();
                camera.goal.translation = player_transform.translation()
                    + distance_from_player
                    + shoulder_shift
                    + 0.7 * Vec3::Y;
                camera.goal.look_to(facing.forward.f32(), Vec3::Y);
                let pitch_axis = camera.goal.left();
                camera.goal.rotate_around(
                    player_transform.translation() + -0.5 * Vec3::Y,
                    Quat::from_axis_angle(*pitch_axis, facing.pitch_angle.f32()),
                );
            }
        }
    } else {
        return;
    };
}

pub(crate) fn switch_first_third_person(
    camera_data: Res<CameraData>,
    mut cameras: Query<(&mut Camera, Option<&FPSCamera>, Option<&TPSCamera>)>,
) {
    let first_person = camera_data.is_first_person();
    for (mut camera, fps, tps) in cameras.iter_mut() {
        camera.is_active = (first_person && fps.is_some()) || (!first_person && tps.is_some());
    }
}

pub(crate) fn hide_player_in_fps(
    camera_mode: Res<CameraData>,
    mut player: Query<&mut Visibility, With<IsPlayer>>,
) {
    *player.get_single_mut().unwrap() = {
        if camera_mode.is_first_person() {
            Visibility::Hidden
        } else {
            Visibility::Visible
        }
    };
}
