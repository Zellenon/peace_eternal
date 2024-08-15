use app_setup_options::{AppSetupConfiguration, ScheduleToUse};
use avian3d::{
    schedule::{Physics, PhysicsSchedule},
    PhysicsPlugins,
};
use bevy::{
    app::{App, FixedUpdate, Startup, Update},
    ecs::schedule::ScheduleLabel,
    prelude::IntoSystemConfigs,
    time::Time,
    DefaultPlugins,
};
use bevy_atmosphere::plugin::AtmospherePlugin;
use bevy_tnua::{control_helpers::TnuaCrouchEnforcerPlugin, prelude::*};
use bevy_tnua_avian3d::*;
use character_animating_systems::animate_humanoids;
use character_control_systems::{
    info_dumping_systems::character_control_info_dumping_system,
    platformer_control_systems::{
        apply_platformer_controls, CharacterMotionConfigForPlatformerDemo,
    },
    ControlPlugin,
};
use dev::DevModePlugin;
use gunplay::GunplayPlugin;
use level_mechanics::LevelMechanicsPlugin;
use levels_setup::{
    camera::setup_cameras, level_switching::LevelSwitchingPlugin, player::setup_player,
    setup_lights,
};
use options::OptionsPlugin;
use sketchpad::sketchpad_system;
use ui::DemoInfoUpdateSystemSet;
use util::{animating::animation_patcher_system, UtilPlugin};

mod app_setup_options;
mod character_animating_systems;
mod character_control_systems;
mod dev;
mod gunplay;
mod level_mechanics;
mod levels_setup;
mod options;
mod sketchpad;
mod ui;
mod util;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    let app_setup_configuration = AppSetupConfiguration::from_environment();
    app.insert_resource(app_setup_configuration.clone());
    match app_setup_configuration.schedule_to_use {
        ScheduleToUse::Update => {
            app.add_plugins(PhysicsPlugins::default())
                .add_plugins(TnuaAvian3dPlugin::default())
                .add_plugins(TnuaControllerPlugin::default());
            // Prevents the character from standing up while obstructed by an obstacle.
            app.add_plugins(TnuaCrouchEnforcerPlugin::default());
        }
        ScheduleToUse::FixedUpdate => {
            app.add_plugins(PhysicsPlugins::new(FixedUpdate))
                .add_plugins(TnuaAvian3dPlugin::new(FixedUpdate))
                .add_plugins(TnuaControllerPlugin::new(FixedUpdate));
            app.add_plugins(TnuaCrouchEnforcerPlugin::new(FixedUpdate));
        }
        ScheduleToUse::PhysicsSchedule => {
            app.add_plugins(PhysicsPlugins::default())
                .insert_resource(Time::new_with(Physics::fixed_hz(144.0)))
                .add_plugins(TnuaAvian3dPlugin::new(PhysicsSchedule))
                .add_plugins(TnuaControllerPlugin::new(PhysicsSchedule));
            app.add_plugins(TnuaCrouchEnforcerPlugin::new(PhysicsSchedule));
        }
    }

    app.add_systems(
        Update,
        character_control_info_dumping_system.in_set(DemoInfoUpdateSystemSet),
    );
    app.add_plugins(ui::DemoUi::<CharacterMotionConfigForPlatformerDemo>::default());
    app.add_plugins(AtmospherePlugin);
    app.add_plugins({
        LevelSwitchingPlugin::new(app_setup_configuration.level_to_load.as_ref())
            .with("Default", levels_setup::setup_level)
    });
    app.add_systems(Startup, (setup_player, setup_lights, setup_cameras));
    app.add_systems(
        match app_setup_configuration.schedule_to_use {
            ScheduleToUse::Update => Update.intern(),
            ScheduleToUse::FixedUpdate => FixedUpdate.intern(),
            ScheduleToUse::PhysicsSchedule => PhysicsSchedule.intern(),
        },
        apply_platformer_controls.in_set(TnuaUserControlsSystemSet),
    );
    app.add_systems(Update, animation_patcher_system)
        .add_systems(Update, animate_humanoids)
        .add_plugins(LevelMechanicsPlugin);

    app.add_plugins((ControlPlugin, GunplayPlugin, OptionsPlugin, UtilPlugin));
    app.add_plugins(DevModePlugin);

    app.add_systems(Update, sketchpad_system);

    app.run();
}
