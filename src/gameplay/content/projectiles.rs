use avian3d::prelude::{Collider, RigidBody, SweptCcd};
use bevy::prelude::{Name, SpatialBundle};
use bevy_composable::{app_impl::ComponentTreeable, tree::ComponentTree};

use crate::{
    asset_setup::models::ModelResources,
    gameplay::gunplay::projectiles::{Projectile, ProjectileImpactBehavior},
    util::Lifespan,
};

use super::LinkedModel;

pub fn basic_bullet() -> ComponentTree {
    (
        Name::new("Bullet"),
        Projectile {
            on_hit: ProjectileImpactBehavior::Die,
            on_impact: ProjectileImpactBehavior::Die,
        },
        SpatialBundle::default(),
        SweptCcd::default(),
        RigidBody::Dynamic,
        Lifespan::default(),
        Collider::sphere(0.05),
        // Sensor,
        // TODO: damage
        LinkedModel::new(ModelResources::basic_bullet),
    )
        .store()
}

pub fn gl_projectile() -> ComponentTree {
    (
        Name::new("GL Projectile"),
        Projectile {
            on_hit: ProjectileImpactBehavior::Die,
            on_impact: ProjectileImpactBehavior::Die,
        },
        RigidBody::Dynamic,
        Lifespan::new(10000),
        Collider::sphere(0.05),
        // TODO: damage
        // TODO:increased gravity
        // TODO: Grenade asset
        LinkedModel::new(ModelResources::basic_bullet),
    )
        .store()
}
