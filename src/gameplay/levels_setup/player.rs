use avian3d::{
    collision::{Collider, CollisionLayers, LayerMask},
    dynamics::rigid_body::{LockedAxes, RigidBody},
};
use bevy::{
    asset::{AssetServer, Assets},
    audio::SpatialListener,
    core::Name,
    ecs::system::{Commands, Res},
    math::Vec3,
    prelude::{ResMut, SpatialBundle, Transform},
    scene::SceneBundle,
};
use bevy_composable::{
    app_impl::{ComplexSpawnable, ComponentTreeable, FuncTreeable},
    wrappers::name,
};
use bevy_hanabi::EffectAsset;
use bevy_tnua::{
    builtins::{TnuaBuiltinCrouch, TnuaBuiltinJump, TnuaBuiltinWalk},
    control_helpers::{
        TnuaCrouchEnforcer, TnuaSimpleAirActionsCounter, TnuaSimpleFallThroughPlatformsHelper,
    },
    controller::TnuaControllerBundle,
    TnuaAnimatingState, TnuaGhostSensor, TnuaToggle,
};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use bevy_tnua_physics_integration_layer::math::{Float, Vector3};

use super::{IsPlayer, LayerNames};
use crate::{
    asset_setup::{models::ModelResources, particles::ParticleTextures},
    dev::ui::{
        self, component_alterbation::CommandAlteringSelectors, info::InfoSource,
        plotting::PlotSource,
    },
    gameplay::{
        content::guns::basic_gun,
        controls::{
            camera_controls::Facing,
            keyboard_receive::{
                create_camera_action_input_manager_bundle,
                create_player_action_input_manager_bundle, create_ui_action_input_manager_bundle,
                create_weapon_swap_input_manager_bundle,
            },
            platformer_control_systems::{
                CharacterMotionConfigForPlatformerDemo, FallingThroughControlScheme,
            },
        },
        gunplay::{
            arms::Arm,
            dummy_gun::{Barrel, DummyGun},
        },
        inventory::{
            components::{
                default_inventory, Inventory, InventorySlot, InventorySlotSettings,
                InventorySlotSize,
            },
            swapping::HoldingInventoryItem,
        },
    },
    graphics::{basic_sparks, smoke_puff, AnimationState, MuzzleFlashFX},
    util::{GltfSceneHandler, SmoothedTransform},
};

use InventorySlotSize::{Large, Medium, Small};
pub(crate) fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    models: Res<ModelResources>,
    mut effects: ResMut<Assets<EffectAsset>>,
    particle_graphics: Res<ParticleTextures>,
) {
    let gun = commands.compose(basic_gun());

    let player_tree = name("Player")
        + (
            IsPlayer,
            SceneBundle {
                scene: asset_server.load("models/player.glb#Scene0"),
                ..Default::default()
            },
            GltfSceneHandler {
                names_from: asset_server.load("models/player.glb"),
            },
            RigidBody::Dynamic,
            Collider::capsule(0.5, 1.0),
            Inventory {
                slots: vec![
                    InventorySlot::new([Large, Medium]),
                    InventorySlot {
                        settings: InventorySlotSettings::new([Medium, Small]),
                        contents: Some(gun),
                    },
                ],
            },
            Facing::default(),
            SpatialListener::new(2.0),
        )
            .store();

    let player = commands.compose(player_tree);
    let mut player = commands.get_entity(player).unwrap();

    player.insert(TnuaControllerBundle::default());

    player.insert(CharacterMotionConfigForPlatformerDemo {
        speed: 10.0,
        walk: TnuaBuiltinWalk {
            float_height: 2.0,
            max_slope: std::f32::consts::FRAC_PI_4,
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

    player.insert((
        create_player_action_input_manager_bundle(),
        create_camera_action_input_manager_bundle(),
        create_ui_action_input_manager_bundle(),
        create_weapon_swap_input_manager_bundle(),
    ));

    let id = player.id();
    let arm = (
        Name::new("PlayerArm"),
        Arm::new(&id),
        SpatialBundle::default(),
        SmoothedTransform {
            smoothing: 20.,
            do_rotate: true,
            do_translate: true,
            rotation_mul: 0.6,
            ..Default::default()
        },
        HoldingInventoryItem::default(),
    )
        .store()
        << (name("DummyGun") + DummyGun.store() + SpatialBundle::store_default()
            << (name("Barrel")
                + (
                    Barrel,
                    SpatialBundle {
                        transform: Transform::default()
                            .with_translation(Vec3::new(-0.01, 0.2, -1.2)),
                        ..Default::default()
                    },
                )
                    .store()
                << ((MuzzleFlashFX).store() + basic_sparks(&mut effects))
                << ((MuzzleFlashFX).store() + smoke_puff(&mut effects, &particle_graphics.smoke))));
    commands.compose(arm);
}
