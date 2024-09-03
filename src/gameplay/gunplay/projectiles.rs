use avian3d::prelude::CollisionStarted;
use bevy::{
    core::Name,
    math::{Quat, Vec3},
    prelude::{
        Commands, Component, DespawnRecursiveExt, Entity, Event, EventReader, EventWriter, Query,
        Reflect, Res, With,
    },
};
use bevy_composable::{app_impl::ComplexSpawnable, tree::ComponentTree};

use crate::{
    asset_setup::primitives::PrimitiveResources,
    util::{instant_force, with_translation},
};

#[derive(Reflect, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum ProjectileImpactBehavior {
    Die,
    Bounce,
}

#[derive(Component, Reflect, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Projectile {
    pub on_hit: ProjectileImpactBehavior,
    pub on_impact: ProjectileImpactBehavior,
}

#[derive(Event, Clone)]
pub struct FireProjectile {
    pub bullet: ComponentTree,
    pub location: Vec3,
    pub rotation: Quat,
}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            on_hit: ProjectileImpactBehavior::Die,
            on_impact: ProjectileImpactBehavior::Die,
        }
    }
}

#[derive(Reflect, Debug, PartialEq, Eq, PartialOrd, Ord, Event)]
pub struct ProjectileCollision {
    bullet: Entity,
    victim: Entity,
}

#[derive(Reflect, Debug, PartialEq, Eq, PartialOrd, Ord, Event)]
pub struct ProjectileClash(pub Entity, pub Entity);

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct Knockback(pub f32);

pub fn spawn_bullets(mut commands: Commands, mut bullets: EventReader<FireProjectile>) {
    for FireProjectile {
        bullet,
        location: position,
        rotation: orientation,
    } in bullets.read()
    {
        commands.compose(
            bullet.clone()
                + with_translation(*position, *orientation, 0.1)
                + instant_force(*orientation, 0.08),
        );
    }
}

pub(super) fn catch_projectile_collisions(
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut projectile_hits: EventWriter<ProjectileCollision>,
    mut projectile_clash: EventWriter<ProjectileClash>,
    projectiles: Query<Entity, With<Projectile>>,
) {
    for CollisionStarted(e1, e2) in collision_event_reader.read() {
        match (projectiles.get(*e1), projectiles.get(*e2)) {
            (Ok(_), Ok(_)) => {
                projectile_clash.send(ProjectileClash(*e1, *e2));
            }
            (Ok(_), Err(_)) => {
                projectile_hits.send(ProjectileCollision {
                    bullet: *e1,
                    victim: *e2,
                });
            }
            (Err(_), Ok(_)) => {
                projectile_hits.send(ProjectileCollision {
                    bullet: *e2,
                    victim: *e1,
                });
            }
            (Err(_), Err(_)) => {}
        };
        println!("Entities {:?} and {:?} started colliding", e1, e2,);
    }
}

pub(super) fn kill_projectiles_on_hit(
    mut collisions: EventReader<ProjectileCollision>,
    names: Query<&Name>,
    mut commands: Commands,
) {
    for collision in collisions.read() {
        // if we're supposed to kill it
        commands
            .get_entity(collision.bullet)
            .unwrap()
            .despawn_recursive();
        println!("Killing {}!", names.get(collision.bullet).unwrap())
    }
}
