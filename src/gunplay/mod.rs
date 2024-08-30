use arms::{do_arm_recoil, do_shake_recoil, update_arm_position, Arm, Recoil};
use bevy::{
    app::{Plugin, Update},
    prelude::IntoSystemConfigs,
    reflect::Reflect,
};
use projectiles::{
    catch_projectile_collisions, kill_projectiles_on_hit, Knockback, Projectile, ProjectileClash,
    ProjectileCollision,
};
use servo::{
    do_should_activate, player_servos_on_click, receive_servo_arming_events, tick_cooldowns,
    ArmServo, Servo, ServoActivated,
};

use crate::{
    asset_setup::primitives::PrimitiveResources,
    character_control_systems::camera_controls::apply_mouse_camera_movement,
};

use self::guns::{fire_guns, Gun};

pub mod arms;
pub mod guns;
pub mod projectiles;
pub mod servo;

#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct GunplayPlugin;

impl Plugin for GunplayPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ArmServo>()
            .add_event::<ServoActivated>()
            .add_event::<Recoil>()
            .add_event::<ProjectileCollision>()
            .add_event::<ProjectileClash>();
        app.insert_resource(PrimitiveResources::default());

        app.register_type::<Servo>()
            .register_type::<Arm>()
            .register_type::<Gun>()
            .register_type::<Projectile>()
            .register_type::<Knockback>();

        app.add_systems(
            Update,
            update_arm_position.after(apply_mouse_camera_movement),
        );

        app.add_systems(
            Update,
            player_servos_on_click.before(receive_servo_arming_events),
        )
        .add_systems(
            Update,
            (tick_cooldowns, receive_servo_arming_events).before(do_should_activate),
        )
        .add_systems(
            Update,
            (
                (do_arm_recoil, do_shake_recoil),
                do_should_activate,
                fire_guns,
            )
                .chain(),
        );

        app.add_systems(
            Update,
            (catch_projectile_collisions, kill_projectiles_on_hit).chain(),
        );
    }
}
