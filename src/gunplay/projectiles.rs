use avian3d::prelude::{CollisionStarted, Sensor};
use bevy::prelude::{
    Commands, Component, DespawnRecursiveExt, Entity, EventReader, EventWriter, Query, With,
};
use bevy::prelude::{Event, Reflect};

#[derive(Reflect, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjectileImpactBehavior {
    Die,
    Bounce,
}

#[derive(Component, Reflect, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Projectile {
    pub on_hit: ProjectileImpactBehavior,
    pub on_impact: ProjectileImpactBehavior,
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

pub(super) fn catch_projectile_collisions(
    mut collision_event_reader: EventReader<CollisionStarted>,
    mut projectile_hits: EventWriter<ProjectileCollision>,
    mut projectile_clash: EventWriter<ProjectileClash>,
    projectiles: Query<Entity, (With<Sensor>, With<Projectile>)>,
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
    mut commands: Commands,
) {
    for collision in collisions.read() {
        // if we're supposed to kill it
        commands
            .get_entity(collision.bullet)
            .unwrap()
            .despawn_recursive()
    }
}
