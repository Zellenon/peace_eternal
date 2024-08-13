pub mod component_alterbation;
mod framerate;
pub mod info;
mod level_selection;
pub mod plotting;
pub mod tuning;

use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
#[allow(unused_imports)]
use bevy_tnua::math::AsF32;
use bevy_tnua::TnuaToggle;

use self::component_alterbation::CommandAlteringSelectors;
use self::plotting::{make_update_plot_data_system, plot_source_rolling_update};

use tuning::UiTunable;

#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub struct DemoInfoUpdateSystemSet;

pub struct DemoUi<C: Component + UiTunable> {
    _phantom: PhantomData<C>,
}

impl<C: Component + UiTunable> Default for DemoUi<C> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<C: Component + UiTunable> Plugin for DemoUi<C> {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin);
        app.insert_resource(DemoUiPhysicsBackendActive(true));
        app.configure_sets(
            Update,
            DemoInfoUpdateSystemSet.after(bevy_tnua::TnuaUserControlsSystemSet),
        );
        app.add_systems(Update, apply_selectors);
        app.add_systems(Update, ui_system::<C>.after(DemoInfoUpdateSystemSet));
        app.add_systems(Update, plot_source_rolling_update);

        app.add_plugins(framerate::DemoFrameratePlugin);

        {
            app.add_systems(
                Update,
                make_update_plot_data_system(
                    |velocity: &avian3d::dynamics::rigid_body::LinearVelocity| (**velocity).f32(),
                ),
            );
        }

        app.add_systems(Update, update_physics_active_from_ui);
    }
}

// NOTE: The demos are responsible for updating the physics backend
#[derive(Resource)]
pub struct DemoUiPhysicsBackendActive(pub bool);

#[derive(Component)]
pub struct TrackedEntity(pub String);

fn apply_selectors(
    mut query: Query<(Entity, &mut CommandAlteringSelectors)>,
    mut commands: Commands,
) {
    for (entity, mut command_altering_selectors) in query.iter_mut() {
        command_altering_selectors.apply_set_to(&mut commands, entity);
    }
}

#[allow(clippy::type_complexity)]
fn ui_system<C: Component + UiTunable>(
    mut egui_context: EguiContexts,
    mut physics_backend_active: ResMut<DemoUiPhysicsBackendActive>,
    mut query: Query<(
        Entity,
        &TrackedEntity,
        Option<&plotting::PlotSource>,
        Option<&mut info::InfoSource>,
        &mut TnuaToggle,
        Option<&mut C>,
        Option<&mut CommandAlteringSelectors>,
    )>,
    mut commands: Commands,
    mut primary_window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut level_selection: level_selection::LevelSelectionParam,
    mut framerate: framerate::DemoFramerateParam,
    #[cfg(target_arch = "wasm32")] app_setup_configuration: Res<
        crate::app_setup_options::AppSetupConfiguration,
    >,
) {
    use std::any::TypeId;

    let Ok(mut primary_window) = primary_window_query.get_single_mut() else {
        return;
    };
    let mut egui_window = egui::Window::new("Tnua");
    if !primary_window.cursor.visible {
        egui_window = egui::Window::new("Tnua")
            .interactable(false)
            .movable(false)
            .resizable(false);
    }
    egui_window.show(egui_context.ctx_mut(), |ui| {
        //if let Some(window) = web_sys::window() {
            //ui.label(format!("URL {:?}", window.location().search()));
            //if ui.button("change").clicked() {
                //let _ = window.location().set_search("yes=no");
            //}
        //}
        #[cfg(target_arch = "wasm32")]
        if let Some(new_schedule) = app_setup_configuration.schedule_to_use.pick_different_option(ui) {
            app_setup_configuration.change_and_reload_page(|cfg| {
                cfg.schedule_to_use = new_schedule;
            });
        }
        egui::ComboBox::from_label("Present Mode (picking unsupported mode will crash the demo)")
            .selected_text(format!("{:?}", primary_window.present_mode))
            .show_ui(ui, |ui| {
                let present_mode = &mut primary_window.present_mode;
                ui.selectable_value(present_mode, PresentMode::AutoVsync, "AutoVsync");
                ui.selectable_value(present_mode, PresentMode::AutoNoVsync, "AutoNoVsync");
                ui.selectable_value(present_mode, PresentMode::Fifo, "Fifo");
                ui.selectable_value(present_mode, PresentMode::FifoRelaxed, "FifoRelaxed");
                ui.selectable_value(present_mode, PresentMode::Immediate, "Immediate");
                ui.selectable_value(present_mode, PresentMode::Mailbox, "Mailbox");
            });
        framerate.show_in_ui(ui);
        egui::CollapsingHeader::new("Controls:")
            .default_open(false)
            .show(ui, |ui| {
                ui.label("Move with the arrow keys or WASD");
                ui.label("Left click to toggle mouse-controlled camera (shooter only)");
                ui.label("Jump with Spacebar (Also with the up arrow also works in 2D)");
                ui.label("Crouch or fall through pink platforms with Ctrl (Also with the down arrow key in 2D)");
                ui.label("Turn in place with Alt (only in 3D)");
                ui.label("Dash with Shift (while moving in a direction)");
            });
        level_selection.show_in_ui(ui);
        ui.checkbox(&mut physics_backend_active.0, "Physics Backend Enabled");
        for (
            entity,
            TrackedEntity(name),
            plot_source,
            mut info_source,
            mut tnua_toggle,
            mut tunable,
            command_altering_selectors,
        ) in query.iter_mut()
        {
            let collapse_state = egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), ui.make_persistent_id(("for-character", entity)), false);

            #[derive(Clone, Copy, PartialEq, Default, Debug)]
            enum ThingToShow {
                #[default]
                Settings,
                Plots,
                Info,
            }

            let thing_to_show_id = ui.make_persistent_id((TypeId::of::<ThingToShow>(), entity));
            let is_open = collapse_state.is_open();
            let mut thing_to_show = ui.memory_mut(|mem| *mem.data.get_temp_mut_or_default::<ThingToShow>(thing_to_show_id));
            let mut set_open = None;

            let mut collapse_state = collapse_state.show_header(ui, |ui| {
                ui.label(name);
                for (possible, option, text) in [
                    (true, ThingToShow::Settings, "settings"),
                    (plot_source.is_some(), ThingToShow::Plots, "plots"),
                    (info_source.is_some(), ThingToShow::Info, "info"),
                ] {
                    let mut selected = is_open && option == thing_to_show;
                    ui.add_enabled_ui(possible, |ui| {
                        if ui.toggle_value(&mut selected, text).changed() {
                            set_open = Some(selected);
                            if selected {
                                thing_to_show = option;
                                ui.memory_mut(|mem| *mem.data.get_temp_mut_or_default::<ThingToShow>(thing_to_show_id) = option);
                            }
                        }
                    });
                }
            });
            if let Some(set_open) = set_open {
                collapse_state.set_open(set_open);
            }

            if let Some(info_source) = info_source.as_mut() {
                info_source.set_active(collapse_state.is_open() && thing_to_show == ThingToShow::Info);
            }

            collapse_state.body(|ui| {
                match thing_to_show {
                    ThingToShow::Settings => {
                        egui::ComboBox::from_label("Toggle Tnua")
                            .selected_text(format!("{:?}", tnua_toggle.as_ref()))
                            .show_ui(ui, |ui| {
                                for option in [
                                    TnuaToggle::Disabled,
                                    TnuaToggle::SenseOnly,
                                    TnuaToggle::Enabled,
                                ] {
                                    let label = format!("{:?}", option);
                                    ui.selectable_value(tnua_toggle.as_mut(), option, label);
                                }
                            });

                        if let Some(tunable) = tunable.as_mut() {
                            tunable.tune(ui);
                        }

                        if let Some(mut command_altering_selectors) = command_altering_selectors
                        {
                            command_altering_selectors.show_ui(ui, &mut commands, entity);
                        }
                    }
                    ThingToShow::Plots => {
                        if let Some(plot_source) = plot_source {
                            plot_source.show(entity, ui);
                        } else {
                            ui.colored_label(egui::Color32::DARK_RED, "No plotting configured for this entity");
                        }
                    }
                    ThingToShow::Info => {
                        if let Some(info_source) = info_source.as_mut() {
                            info_source.show(entity, ui);
                        } else {
                            ui.colored_label(egui::Color32::DARK_RED, "No info configured for this entity");
                        }
                    }
                }
            });
        }
    });
}

fn update_physics_active_from_ui(
    setting_from_ui: Res<DemoUiPhysicsBackendActive>,
    mut physics_time_avian3d: Option<ResMut<Time<avian3d::schedule::Physics>>>,
) {
    if let Some(physics_time) = physics_time_avian3d.as_mut() {
        use avian3d::schedule::PhysicsTime;
        if setting_from_ui.0 {
            physics_time.unpause();
        } else {
            physics_time.pause();
        }
    }
}
