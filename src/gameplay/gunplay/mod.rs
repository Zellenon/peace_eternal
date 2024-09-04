use arms::{do_arm_recoil, do_shake_recoil, update_arm_position, Arm, Recoil};
use bevy::{
    app::{Plugin, Update},
    prelude::IntoSystemConfigs,
    reflect::Reflect,
};
use dummy_gun::{
    hide_gun_on_empty_hand, swap_dummygun_model, swap_held_dummy_model, Barrel, DummyGun,
    SwapDummyModel,
};
use guns::{
    dummy_activations_to_inventory_guns, gun_recoil_to_arm, gun_recoil_to_arm2,
    unmirror_gun_activations, DummyMirror, FireGun, RecoilMirror,
};
use projectiles::{
    catch_projectile_collisions, spawn_bullets, FireProjectile, Knockback, Projectile,
    ProjectileClash, ProjectileCollision,
};
use servo::{
    do_directed_servos, do_should_activate, player_servos_on_click, receive_servo_arming_events,
    tick_cooldowns, ArmServo, DirectedServoActivated, Servo, ServoActivated,
};
use servo_components::{
    do_activation_sounds, gunshots_spawn_muzzlefx, gunshots_to_bullet_spawn,
    gunshots_to_muzzle_flare, gunshots_to_muzzle_flash, gunshots_to_recoil, HasActivationSound,
    HasGunSmoke, HasMuzzleFlare, HasMuzzleFlash, HasRecoil, MultiActivation,
};

use crate::{
    asset_setup::primitives::PrimitiveResources,
    gameplay::controls::camera_controls::apply_mouse_camera_movement,
};

use self::guns::Gun;

pub mod arms;
pub mod dummy_gun;
pub mod guns;
pub mod projectiles;
pub mod servo;
pub mod servo_components;

#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct GunplayPlugin;

impl Plugin for GunplayPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ArmServo>()
            .add_event::<ServoActivated>()
            .add_event::<DirectedServoActivated>()
            .add_event::<Recoil>()
            .add_event::<FireGun>()
            .add_event::<FireProjectile>()
            .add_event::<ProjectileCollision>()
            .add_event::<SwapDummyModel>()
            .add_event::<DummyMirror>()
            .add_event::<RecoilMirror>()
            .add_event::<ProjectileClash>();
        app.insert_resource(PrimitiveResources::default());

        app.register_type::<Servo>()
            .register_type::<Arm>()
            .register_type::<Gun>()
            .register_type::<Projectile>()
            .register_type::<Barrel>()
            .register_type::<DummyGun>()
            .register_type::<FireGun>()
            // .register_type::<ShootsBullets>()
            .register_type::<HasMuzzleFlare>()
            .register_type::<HasMuzzleFlash>()
            .register_type::<HasGunSmoke>()
            .register_type::<HasRecoil>()
            .register_type::<HasActivationSound>()
            .register_type::<MultiActivation>()
            .register_type::<HasRecoil>()
            .register_type::<Knockback>();

        app.add_systems(
            Update,
            update_arm_position.after(apply_mouse_camera_movement),
        );

        app.add_systems(
            Update,
            (
                swap_held_dummy_model,
                swap_dummygun_model,
                hide_gun_on_empty_hand,
            )
                .chain(),
        );

        app.add_systems(
            Update,
            (
                player_servos_on_click,
                (tick_cooldowns, receive_servo_arming_events),
                do_should_activate.before(do_directed_servos),
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                (
                    gun_recoil_to_arm,
                    gun_recoil_to_arm2,
                    (do_arm_recoil, do_shake_recoil),
                )
                    .chain(),
                do_directed_servos,
                dummy_activations_to_inventory_guns,
                unmirror_gun_activations,
                (
                    gunshots_to_bullet_spawn,
                    gunshots_to_muzzle_flare,
                    gunshots_to_muzzle_flash,
                    gunshots_spawn_muzzlefx,
                    do_activation_sounds,
                    gunshots_to_recoil,
                ),
            )
                .chain(),
        );

        app.add_systems(Update, spawn_bullets.after(gunshots_to_bullet_spawn));

        app.add_systems(
            Update,
            (
                catch_projectile_collisions,
                // kill_projectiles_on_hit
            )
                .chain(),
        );
    }
}
