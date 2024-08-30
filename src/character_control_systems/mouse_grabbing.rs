use bevy::prelude::Reflect;
use std::ops::Not;

use bevy::{
    ecs::{
        query::With,
        system::{Query, Res, ResMut, Resource},
    },
    input::{mouse::MouseButton, ButtonInput},
    window::{CursorGrabMode, PrimaryWindow, Window},
};
use leafwing_input_manager::action_state::ActionState;

use super::keyboard_receive::UiAction;

#[derive(Resource, Reflect, Clone, Debug, PartialEq)]
pub struct MouseGrabbed(pub bool);

pub(crate) fn sync_mouse_grab(
    grab_resource: Res<MouseGrabbed>,
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

pub(crate) fn grab_mouse_on_click(
    egui_context: bevy_egui::EguiContexts,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
    mut grab_resource: ResMut<MouseGrabbed>,
) {
    if let Ok(window) = primary_window_query.get_single() {
        if window.cursor.visible
            && mouse_buttons.just_pressed(MouseButton::Left)
            && egui_context.ctx().is_pointer_over_area().not()
        {
            grab_resource.0 = true;
        } else {
            return;
        }
    };
}

pub(crate) fn release_mouse_in_inventory(
    ui_actions: Query<&ActionState<UiAction>>,
    mut grab_resource: ResMut<MouseGrabbed>,
) {
    if ui_actions
        .get_single()
        .unwrap()
        .just_pressed(&UiAction::ToggleInventory)
    {
        grab_resource.0 = false;
    }
}
