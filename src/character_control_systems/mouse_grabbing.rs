use std::ops::Not;

use bevy::{
    ecs::{
        event::{Event, EventReader, EventWriter},
        query::With,
        system::{Query, Res, ResMut, Resource},
    },
    input::{keyboard::KeyCode, mouse::MouseButton, ButtonInput},
    window::{CursorGrabMode, PrimaryWindow, Window},
};

#[derive(Resource)]
pub struct MouseGrabbed(pub bool);

pub(crate) fn sync_mouse_grab(
    mut grab_resource: ResMut<MouseGrabbed>,
    mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let Ok(mut window) = primary_window_query.get_single_mut() else {
        return;
    };
    if grab_resource.0 {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    } else {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

// TODO: eguicontexts shouldn't be mut, but for some reason the ctx() function doesn't exist??
pub(crate) fn grab_mouse_on_click(
    mut egui_context: bevy_egui::EguiContexts,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mut grab_resource: ResMut<MouseGrabbed>,
) {
    if let Ok(window) = primary_window_query.get_single() {
        if window.cursor.visible
            && mouse_buttons.just_pressed(MouseButton::Left)
            && egui_context.ctx_mut().is_pointer_over_area().not()
        {
            grab_resource.0 = true;
            print!("Grabbing mouse!");
        } else {
            return;
        }
    };
}
