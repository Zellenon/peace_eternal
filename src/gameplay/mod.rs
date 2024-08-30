pub mod character_control_systems;
pub mod content;
pub mod guardrails;
mod gunplay;
pub mod level_mechanics;
pub mod levels_setup;

#[allow(unused_imports)]
pub use gunplay::{arms::Arm, guns::Gun, projectiles::Projectile, servo::Servo, GunplayPlugin};

#[allow(unused)]
fn potato() {
    // let a =
}
