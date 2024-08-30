use avian3d::debug_render::PhysicsGizmos;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_editor_pls::prelude::*;

use self::editor::{set_cursor_grab_mode, DevEditorState, DevEditorWindow};
use crate::gameplay::character_control_systems::platformer_control_systems::CharacterMotionConfigForPlatformerDemo;
use ui::DemoUi;

pub(crate) mod editor;
pub(crate) mod ui;

pub struct DevModePlugin;

impl Plugin for DevModePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EditorPlugin::new())
            .insert_resource(default_editor_controls())
            .add_plugins((
                // FrameTimeDiagnosticsPlugin,
                LogDiagnosticsPlugin::filtered(vec![]),
                // PhysicsDebugPlugin::default(),
            ))
            .insert_gizmo_config(
                PhysicsGizmos {
                    aabb_color: Some(Color::WHITE),
                    ..default()
                },
                GizmoConfig {
                    enabled: false,
                    ..default()
                },
            );

        app.add_plugins(DemoUi::<CharacterMotionConfigForPlatformerDemo>::default());

        app.init_resource::<DevEditorState>()
            .add_editor_window::<DevEditorWindow>()
            .add_systems(
                Update,
                set_cursor_grab_mode, //handle_debug_render.pipe(error),
            );
    }
}

fn default_editor_controls() -> bevy_editor_pls::controls::EditorControls {
    use bevy_editor_pls::controls::*;
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(Action::PlayPauseEditor);
    editor_controls.insert(
        Action::PlayPauseEditor,
        Binding {
            input: UserInput::Single(Button::Keyboard(KeyCode::KeyQ)),
            conditions: vec![BindingCondition::ListeningForText(false)],
        },
    );
    editor_controls
}
