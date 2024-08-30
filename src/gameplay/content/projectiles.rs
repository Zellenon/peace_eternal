use avian3d::prelude::{Collider, RigidBody, SweptCcd};
use bevy::{
    asset::Handle,
    pbr::{PbrBundle, StandardMaterial},
    prelude::{Mesh, Name},
};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};

use crate::{
    gameplay::gunplay::projectiles::{Projectile, ProjectileImpactBehavior},
    util::Lifespan,
};

pub fn basic_bullet(mesh: &Handle<Mesh>, material: &Handle<StandardMaterial>) -> ComponentTree {
    let mesh = mesh.clone();
    let material = material.clone();
    (
        Name::new("Bullet"),
        Projectile {
            on_hit: ProjectileImpactBehavior::Die,
            on_impact: ProjectileImpactBehavior::Die,
        },
        PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            ..Default::default()
        },
        SweptCcd::default(),
        RigidBody::Dynamic,
        Lifespan::default(),
        Collider::sphere(0.5),
        // Sensor,
    )
        .store()
}
