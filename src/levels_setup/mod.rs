mod helper;
pub mod level_switching;

pub use level_switching::{IsPlayer, LevelObject, PositionPlayer};

use avian3d::prelude::PhysicsLayer;
use avian3d::{
    collision::{Collider, CollisionLayers, Sensor},
    dynamics::rigid_body::RigidBody,
};
use bevy::{color::palettes::css, prelude::*};

#[allow(unused_imports)]
use bevy_tnua::math::{AdjustPrecision, Vector3};
use bevy_tnua::TnuaGhostPlatform;

use crate::level_mechanics::MovingPlatform;

use helper::{LevelSetupHelper3d, LevelSetupHelper3dEntityCommandsExtension};

#[derive(PhysicsLayer)]
pub enum LayerNames {
    Player,
    FallThrough,
    PhaseThrough,
}

pub fn setup_level(mut helper: LevelSetupHelper3d) {
    helper.spawn(PositionPlayer::from(Vec3::new(0.0, 10.0, 0.0)));

    helper.spawn_floor(css::WHITE);

    let mut obstacles_helper = helper.with_color(css::GRAY);
    obstacles_helper.spawn_cuboid(
        "Moderate Slope",
        Transform::from_xyz(7.0, 7.0, 0.0).with_rotation(Quat::from_rotation_z(0.6)),
        Vector3::new(10.0, 0.1, 2.0),
    );
    obstacles_helper.spawn_cuboid(
        "Steep Slope",
        Transform::from_xyz(14.0, 14.0, 0.0).with_rotation(Quat::from_rotation_z(1.0)),
        Vector3::new(10.0, 0.1, 2.0),
    );
    obstacles_helper.spawn_cuboid(
        "Box to Step on",
        Transform::from_xyz(-4.0, 1.0, 0.0),
        Vector3::new(4.0, 2.0, 2.0),
    );
    obstacles_helper.spawn_cuboid(
        "Floating Box",
        Transform::from_xyz(-10.0, 4.0, 0.0),
        Vector3::new(6.0, 1.0, 2.0),
    );
    obstacles_helper.spawn_cuboid(
        "Box to Crawl Under",
        Transform::from_xyz(0.0, 2.6, -5.0),
        Vector3::new(6.0, 1.0, 2.0),
    );

    // Fall-through platforms
    let mut fall_through_obstacles_helper = helper.with_color(css::PINK.with_alpha(0.8));
    for (i, y) in [2.0, 4.5].into_iter().enumerate() {
        let mut cmd = fall_through_obstacles_helper.spawn_cuboid(
            format!("Fall Through #{}", i + 1),
            Transform::from_xyz(6.0, y, 10.0),
            Vector3::new(6.0, 0.5, 2.0),
        );
        {
            cmd.insert(CollisionLayers::new(
                [LayerNames::FallThrough],
                [LayerNames::FallThrough],
            ));
        }
        cmd.insert(TnuaGhostPlatform);
    }

    helper
        .spawn_scene_cuboid(
            "Collision Groups",
            "collision-groups-text.glb#Scene0",
            Transform::from_xyz(10.0, 2.0, 1.0),
            Vector3::new(4.0, 2.0, 4.0),
        )
        .insert(((
            RigidBody::Static,
            Collider::cuboid(4.0, 2.0, 4.0),
            CollisionLayers::new([LayerNames::PhaseThrough], [LayerNames::PhaseThrough]),
        ),));

    helper
        .spawn_scene_cuboid(
            "Sensor",
            "sensor-text.glb#Scene0",
            Transform::from_xyz(20.0, 2.0, 1.0),
            Vector3::new(4.0, 2.0, 4.0),
        )
        .insert((Sensor,));

    // spawn moving and spinning platforms
    let mut moving_platform_helper = helper.with_color(css::BLUE);
    moving_platform_helper
        .spawn_cuboid(
            "Moving Platform",
            Transform::from_xyz(-4.0, 6.0, 0.0),
            Vector3::new(4.0, 1.0, 4.0),
        )
        .make_kinematic()
        .insert(MovingPlatform::new(
            4.0,
            &[
                Vector3::new(-4.0, 6.0, 0.0),
                Vector3::new(-8.0, 6.0, 0.0),
                Vector3::new(-8.0, 10.0, 0.0),
                Vector3::new(-8.0, 10.0, -4.0),
                Vector3::new(-4.0, 10.0, -4.0),
                Vector3::new(-4.0, 10.0, 0.0),
            ],
        ));

    moving_platform_helper
        .spawn_cylinder(
            "Spinning Platform",
            Transform::from_xyz(-2.0, 2.0, 10.0),
            3.0,
            0.5,
        )
        .make_kinematic_with_angular_velocity(Vector3::Y);
}
