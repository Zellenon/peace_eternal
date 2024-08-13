use bevy::{
    ecs::{
        event::EventReader,
        query::{QuerySingleError, With},
        system::{Query, Res},
        world::Mut,
    },
    input::{
        keyboard::KeyCode,
        mouse::{MouseButton, MouseMotion},
        ButtonInput,
    },
    math::{Quat, Vec2, Vec3},
    render::camera::Camera,
    transform::components::{GlobalTransform, Transform},
    window::{CursorGrabMode, PrimaryWindow, Window},
};
use bevy_tnua::math::{float_consts, AdjustPrecision, AsF32, Quaternion};

use crate::options::controls::ControlOptions;

use super::platformer_control_systems::ForwardFromCamera;

pub fn grab_ungrab_mouse(
    mut egui_context: bevy_egui::EguiContexts,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut window) = primary_window_query.get_single_mut() else {
        return;
    };
    if window.cursor.visible {
        if mouse_buttons.just_pressed(MouseButton::Left) {
            if egui_context.ctx_mut().is_pointer_over_area() {
                return;
            }
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        }
    } else if keyboard.just_released(KeyCode::Escape)
        || mouse_buttons.just_pressed(MouseButton::Left)
    {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

pub fn apply_camera_controls(
    player_camera_movement: Query<&Window, With<PrimaryWindow>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut player_character_query: Query<(&GlobalTransform, &mut ForwardFromCamera)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    control_options: Res<ControlOptions>,
) {
    let is_mouse_controlling_camera = player_camera_movement
        .get_single()
        .map_or(false, |w| !w.cursor.visible);
    let total_mouse_movement = if is_mouse_controlling_camera {
        mouse_motion.read().map(|event| event.delta).sum()
    } else {
        mouse_motion.clear();
        Vec2::ZERO
    } * control_options.mouse_sensitivity;
    let Ok((player_transform, mut forward_from_camera )): Result<(&GlobalTransform, Mut<'_, ForwardFromCamera>), QuerySingleError> = player_character_query.get_single_mut()
    else {
        return;
    };

    let x_shift = Quaternion::from_rotation_y(-0.01 * total_mouse_movement.x.adjust_precision());
    forward_from_camera.forward = x_shift.mul_vec3(forward_from_camera.forward);

    let pitch = 0.005 * total_mouse_movement.y.adjust_precision();
    forward_from_camera.pitch_angle = (forward_from_camera.pitch_angle + pitch)
        .clamp(-float_consts::FRAC_PI_2, float_consts::FRAC_PI_2);

    for mut camera in camera_query.iter_mut() {
        camera.translation = player_transform.translation()
            + -5.0 * forward_from_camera.forward.f32()
            + forward_from_camera.forward.cross(Vec3::Y).f32()
            + 0.75 * Vec3::Y;
        camera.look_to(forward_from_camera.forward.f32(), Vec3::Y);
        let pitch_axis = camera.left();
        camera.rotate_around(
            player_transform.translation() + -0.5 * Vec3::Y,
            Quat::from_axis_angle(*pitch_axis, forward_from_camera.pitch_angle.f32()),
        );
    }
}
