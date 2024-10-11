use core::f64;

use avian3d::parry::na::ComplexField;
use bevy::{
    math::{Quat, Vec3},
    prelude::{Commands, Event, EventReader, Name, Res, Transform},
    reflect::Reflect,
    scene::SceneBundle,
};
use bevy_composable::{
    app_impl::{ComplexSpawnable, ComponentTreeable},
    tree::ComponentTree,
};
use bevy_egui::egui::emath::Numeric;
use bevy_tnua::math::AsF32;

use crate::{
    asset_setup::models::ModelResources,
    util::{GltfSceneHandler, ShrinkDeath},
};

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
pub struct SpawnMuzzleFlare {
    pub location: Vec3,
    pub size: f32,
    pub direction: Quat,
    pub petals: usize,
    pub petal_coef: f32,
}

fn muzzle_flare(
    location: &Vec3,
    size: &f32,
    direction: &Quat,
    models: &ModelResources,
) -> ComponentTree {
    ((
        Name::new("Flare"),
        SceneBundle {
            scene: models.flare_scene.clone(),
            transform: Transform {
                translation: *location,
                rotation: *direction,
                scale: Vec3::splat(*size),
            },
            ..Default::default()
        },
        GltfSceneHandler {
            names_from: models.flare_names.clone(),
        },
        // ShrinkDeath(0.1),
    ))
        .store()
}

pub(super) fn spawn_flare(
    mut events: EventReader<SpawnMuzzleFlare>,
    mut commands: Commands,
    models: Res<ModelResources>,
) {
    for SpawnMuzzleFlare {
        location,
        size,
        direction,
        petals,
        petal_coef,
    } in events.read()
    {
        commands.compose(muzzle_flare(location, size, direction, &*models));

        if *petals > 0 {
            let starting_angle = f64::consts::PI * (petals - 1).to_f64().powf(0.2);
            let petal_size = petal_coef * size;
            let angle_off_center = (f64::consts::PI / -8.) as f32;
            for i in 0..*petals {
                let clock_angle =
                    starting_angle + (i.to_f64() * f64::consts::TAU / petals.to_f64());
                let direction = (*direction * Quat::from_rotation_y(f64::consts::FRAC_PI_2.f32()))
                    * Quat::from_rotation_x(clock_angle.f32())
                    * Quat::from_rotation_y(angle_off_center);
                println!("Clock angle: {}, Direction: {}", clock_angle, direction);
                commands.compose(muzzle_flare(location, &petal_size, &direction, &*models));
            }
        }
    }
}
