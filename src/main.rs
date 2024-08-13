use avian3d::{
    collision::{Collider, CollisionLayers, LayerMask},
    dynamics::rigid_body::{LockedAxes, RigidBody},
    schedule::{Physics, PhysicsSchedule},
    PhysicsPlugins,
};
use bevy::{
    app::{App, FixedUpdate, Startup, Update},
    asset::AssetServer,
    core_pipeline::core_3d::Camera3dBundle,
    ecs::{
        schedule::ScheduleLabel,
        system::{Commands, Res},
    },
    math::Vec3,
    pbr::{DirectionalLight, DirectionalLightBundle, PointLightBundle},
    prelude::{default, IntoSystemConfigs},
    scene::SceneBundle,
    time::Time,
    transform::components::Transform,
    DefaultPlugins,
};
use bevy_tnua::{
    builtins::TnuaBuiltinCrouch,
    control_helpers::{
        TnuaCrouchEnforcer, TnuaCrouchEnforcerPlugin, TnuaSimpleAirActionsCounter,
        TnuaSimpleFallThroughPlatformsHelper,
    },
    math::{float_consts, Float, Vector3},
    prelude::*,
    TnuaAnimatingState, TnuaGhostSensor, TnuaToggle,
};
use bevy_tnua_avian3d::*;

use app_setup_options::{AppSetupConfiguration, ScheduleToUse};
use character_animating_systems::{animate_humanoids, AnimationState};
use character_control_systems::{
    info_dumping_systems::character_control_info_dumping_system,
    platformer_control_systems::{
        apply_platformer_controls, CharacterMotionConfigForPlatformerDemo,
        FallingThroughControlScheme, ForwardFromCamera,
    },
    ControlPlugin,
};
use level_mechanics::LevelMechanicsPlugin;
use levels_setup::{level_switching::LevelSwitchingPlugin, IsPlayer, LayerNames};
use options::OptionsPlugin;
use ui::{
    component_alterbation::CommandAlteringSelectors, info::InfoSource, plotting::PlotSource,
    DemoInfoUpdateSystemSet,
};
use util::animating::{animation_patcher_system, GltfSceneHandler};

mod app_setup_options;
mod character_animating_systems;
mod character_control_systems;
mod level_mechanics;
mod levels_setup;
mod options;
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
    app.add_systems(Startup, setup_camera_and_lights);
    app.add_plugins({
        LevelSwitchingPlugin::new(app_setup_configuration.level_to_load.as_ref())
            .with("Default", levels_setup::setup_level)
    });
    app.add_systems(Startup, setup_player);
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

    app.add_plugins((OptionsPlugin, ControlPlugin));
    app.run();
}

fn setup_camera_and_lights(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 16.0, 40.0)
            .looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(5.0, 5.0, 5.0),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 4000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::default().looking_at(Vec3::new(-0.1, -1., -0.4), Vec3::Z),
        ..Default::default()
    });
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut player = commands.spawn(IsPlayer);
    player.insert(SceneBundle {
        scene: asset_server.load("player.glb#Scene0"),
        ..Default::default()
    });
    player.insert(GltfSceneHandler {
        names_from: asset_server.load("player.glb"),
    });

    player.insert(RigidBody::Dynamic);
    player.insert(Collider::capsule(0.5, 1.0));

    // This bundle container `TnuaController` - the main interface of Tnua with the user code - as
    // well as the main components used as API between the main plugin and the physics backend
    // integration. These components (and the IO bundle, in case of backends that need one like
    // Rapier) are the only mandatory Tnua components - but this example will also add some
    // components used for more advanced features.
    //
    // Read examples/src/character_control_systems/platformer_control_systems.rs to see how
    // `TnuaController` is used in this example.
    player.insert(TnuaControllerBundle::default());

    player.insert(CharacterMotionConfigForPlatformerDemo {
        speed: 10.0,
        walk: TnuaBuiltinWalk {
            float_height: 2.0,
            max_slope: float_consts::FRAC_PI_4,
            turning_angvel: Float::INFINITY,
            acceleration: 50.,
            air_acceleration: 10.,

            // cling_distance: todo!(),
            // spring_strengh: todo!(),
            // spring_dampening: todo!(),
            // coyote_time: todo!(),
            // free_fall_extra_gravity: todo!(),
            // tilt_offset_angvel: todo!(),
            // tilt_offset_angacl: todo!(),
            ..Default::default()
        },
        actions_in_air: 0,
        jump: TnuaBuiltinJump {
            height: 2.0,
            ..Default::default()
        },
        crouch: TnuaBuiltinCrouch {
            float_offset: -0.9,
            ..Default::default()
        },
        dash_distance: 10.0,
        dash: Default::default(),
        one_way_platforms_min_proximity: 1.0,
        falling_through: FallingThroughControlScheme::SingleFall,
    });

    player.insert(ForwardFromCamera::default());

    // An entity's Tnua behavior can be toggled individually with this component, if inserted.
    player.insert(TnuaToggle::default());

    // This is an helper component for deciding which animation to play. Tnua itself does not
    // actually interact with `TnuaAnimatingState` - it's there so that animating systems could use
    // the information from `TnuaController` to animate the character.
    //
    // Read examples/src/character_animating_systems/platformer_animating_systems.rs to see how
    // `TnuaAnimatingState` is used in this example.
    player.insert(TnuaAnimatingState::<AnimationState>::default());

    player.insert({
        let command_altering_selectors = CommandAlteringSelectors::default()
            // By default Tnua uses a raycast, but this could be a problem if the character stands
            // just past the edge while part of its body is above the platform. To solve this, we
            // need to cast a shape - which is physics-engine specific. We set the shape using a
            // component.
            .with_combo(
                "Sensor Shape",
                1,
                &[
                    ("no", |mut cmd| {
                        cmd.remove::<TnuaAvian3dSensorShape>();
                    }),
                    ("flat (underfit)", |mut cmd| {
                        cmd.insert(TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)));
                    }),
                    ("flat (exact)", |mut cmd| {
                        cmd.insert(TnuaAvian3dSensorShape(Collider::cylinder(0.5, 0.0)));
                    }),
                    ("flat (overfit)", |mut cmd| {
                        cmd.insert(TnuaAvian3dSensorShape(Collider::cylinder(0.51, 0.0)));
                    }),
                    ("ball (underfit)", |mut cmd| {
                        cmd.insert(TnuaAvian3dSensorShape(Collider::sphere(0.49)));
                    }),
                    ("ball (exact)", |mut cmd| {
                        cmd.insert(TnuaAvian3dSensorShape(Collider::sphere(0.5)));
                    }),
                ],
            )
            .with_checkbox("Lock Tilt", true, |mut cmd, lock_tilt| {
                // Tnua will automatically apply angular impulses/forces to fix the tilt and make
                // the character stand upward, but it is also possible to just let the physics
                // engine prevent rotation (other than around the Y axis, for turning)
                if lock_tilt {
                    cmd.insert(LockedAxes::new().lock_rotation_x().lock_rotation_z());
                } else {
                    cmd.insert(LockedAxes::new());
                }
            })
            .with_checkbox(
                "Phase Through Collision Groups",
                true,
                |mut cmd, use_collision_groups| {
                    let player_layers: LayerMask = if use_collision_groups {
                        [LayerNames::Player].into()
                    } else {
                        [LayerNames::Player, LayerNames::PhaseThrough].into()
                    };
                    cmd.insert(CollisionLayers::new(player_layers, player_layers));
                },
            );
        command_altering_selectors
    });

    // `TnuaCrouchEnforcer` can be used to prevent the character from standing up when obstructed.
    player.insert(TnuaCrouchEnforcer::new(0.5 * Vector3::Y, |cmd| {
        cmd.insert(TnuaAvian3dSensorShape(Collider::cylinder(0.5, 0.0)));
    }));

    // The ghost sensor is used for detecting ghost platforms - platforms configured in the physics
    // backend to not contact with the character (or detect the contact but not apply physical
    // forces based on it) and marked with the `TnuaGhostPlatform` component. These can then be
    // used as one-way platforms.
    player.insert(TnuaGhostSensor::default());

    // This helper is used to operate the ghost sensor and ghost platforms and implement
    // fall-through behavior where the player can intentionally fall through a one-way platform.
    player.insert(TnuaSimpleFallThroughPlatformsHelper::default());

    // This helper keeps track of air actions like jumps or air dashes.
    player.insert(TnuaSimpleAirActionsCounter::default());

    player.insert((
        ui::TrackedEntity("Player".to_owned()),
        PlotSource::default(),
        InfoSource::default(),
    ));
}
