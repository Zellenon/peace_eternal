use crate::util::deathmarker::Lifespan;
use avian3d::prelude::{Collider, RigidBody, SweptCcd};
use bevy::{
    asset::Handle,
    pbr::{PbrBundle, StandardMaterial},
    prelude::{Mesh, Name},
};
use bevy_composable::{
    tree::{ComponentTree, EntityCommandSet},
    CT,
};

pub fn basic_bullet(mesh: &Handle<Mesh>, material: &Handle<StandardMaterial>) -> ComponentTree {
    let mesh = mesh.clone();
    let material = material.clone();
    CT!(
        Name::new("Bullet"),
        PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            ..Default::default()
        },
        SweptCcd::default(),
        RigidBody::Dynamic,
        // Lifespan::default(),
        Collider::sphere(0.5)
    )
}
