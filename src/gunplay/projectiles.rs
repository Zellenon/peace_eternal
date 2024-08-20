use bevy::prelude::Component;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum ProjectileImpactBehavior {
    Die,
    Bounce,
}

#[derive(Component)]
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

#[derive(Component)]
pub struct Knockback(pub f32);
