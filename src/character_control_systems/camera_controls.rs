use bevy::{
    ecs::{
        event::EventReader,
        query::{QuerySingleError, With},
        system::{Query, Res},
        world::Mut,
    },
    input::mouse::MouseMotion,
    math::{Quat, Vec2, Vec3},
    render::camera::Camera,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};
use bevy_tnua::math::{float_consts, AdjustPrecision, AsF32, Quaternion};

use crate::options::controls::ControlOptions;

use super::platformer_control_systems::ForwardFromCamera;

pub fn mouse_should_control_camera(
    player_camera_movement: Query<&Window, With<PrimaryWindow>>,
) -> bool {
    player_camera_movement
        .get_single()
        .map_or(false, |w| !w.cursor.visible)
}

pub fn apply_mouse_camera_movement(
    mut mouse_motion: EventReader<MouseMotion>,
    mut player_character_query: Query<&mut ForwardFromCamera>,
    control_options: Res<ControlOptions>,
) {
    let total_movement: Vec2 = mouse_motion.read().map(|event| event.delta).sum::<Vec2>()
        * control_options.mouse_sensitivity;
    if let Ok(mut forward_from_camera) = player_character_query.get_single_mut() {
        let x_shift = Quaternion::from_rotation_y(-0.01 * total_movement.x.adjust_precision());
        forward_from_camera.forward = x_shift.mul_vec3(forward_from_camera.forward);

        let pitch = 0.005 * total_movement.y.adjust_precision();
        forward_from_camera.pitch_angle = (forward_from_camera.pitch_angle + pitch)
            .clamp(-float_consts::FRAC_PI_2, float_consts::FRAC_PI_2);
    }
}

pub(crate) fn camera_follow_player(
    player_character_query: Query<(&GlobalTransform, &ForwardFromCamera)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if let Ok((player_transform, forward_from_camera)) = player_character_query.get_single() {
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
    } else {
        return;
    };
}
