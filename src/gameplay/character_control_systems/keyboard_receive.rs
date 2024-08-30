use bevy::{
    input::mouse::MouseButton,
    prelude::{default, KeyCode, Vec2},
    reflect::Reflect,
};
use leafwing_input_manager::{
    action_state::DualAxisData,
    prelude::{InputMap, KeyboardVirtualDPad, MouseMove, MouseScrollAxis},
    Actionlike, InputManagerBundle,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect, Default)]
pub(crate) enum PlayerAction {
    #[default]
    Walk,
    Sprint,
    Crouch,
    Jump,
    Interact,
    Shoot,
}

impl Actionlike for PlayerAction {
    fn input_control_kind(&self) -> leafwing_input_manager::prelude::InputControlKind {
        match self {
            PlayerAction::Walk => leafwing_input_manager::InputControlKind::DualAxis,
            _ => leafwing_input_manager::InputControlKind::Button,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, Default)]
pub(crate) enum CameraAction {
    #[default]
    Orbit,
    Zoom,
}

impl Actionlike for CameraAction {
    fn input_control_kind(&self) -> leafwing_input_manager::prelude::InputControlKind {
        match self {
            CameraAction::Zoom => leafwing_input_manager::InputControlKind::Axis,
            CameraAction::Orbit => leafwing_input_manager::InputControlKind::DualAxis,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Actionlike, Reflect, Default)]
pub(crate) enum UiAction {
    #[default]
    TogglePause,
    ToggleInventory,
}

pub(crate) fn create_player_action_input_manager_bundle() -> InputManagerBundle<PlayerAction> {
    InputManagerBundle {
        input_map: InputMap::new([
            (PlayerAction::Jump, KeyCode::Space),
            (PlayerAction::Sprint, KeyCode::ShiftLeft),
            (PlayerAction::Interact, KeyCode::KeyE),
            (PlayerAction::Crouch, KeyCode::ControlLeft),
        ])
        .with(PlayerAction::Shoot, MouseButton::Left)
        .with_dual_axis(PlayerAction::Walk, KeyboardVirtualDPad::WASD),
        ..default()
    }
}

pub(crate) fn create_camera_action_input_manager_bundle() -> InputManagerBundle<CameraAction> {
    InputManagerBundle {
        input_map: InputMap::default()
            .with_dual_axis(CameraAction::Orbit, MouseMove::default())
            .with_axis(CameraAction::Zoom, MouseScrollAxis::Y),
        ..default()
    }
}

pub(crate) fn create_ui_action_input_manager_bundle() -> InputManagerBundle<UiAction> {
    InputManagerBundle {
        input_map: InputMap::new([
            (UiAction::TogglePause, KeyCode::Escape),
            (UiAction::ToggleInventory, KeyCode::Tab),
        ]),
        ..default()
    }
}

pub(crate) trait DualAxisDataExt {
    fn max_normalized(self) -> Option<Vec2>;
}

impl DualAxisDataExt for DualAxisData {
    fn max_normalized(self) -> Option<Vec2> {
        let vector = self.pair;
        let len_squared = vector.length_squared();
        if len_squared > 1.0 {
            Some(vector.normalize())
        } else if len_squared < 1e-5 {
            None
        } else {
            Some(vector)
        }
    }
}
