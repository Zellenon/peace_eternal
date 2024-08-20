use arms::{do_arm_recoil, do_shake_recoil, update_arm_position, Arm, Recoil};
use bevy::app::{Plugin, Update};
use bevy::prelude::IntoSystemConfigs;
use servo::{
    do_should_activate, player_servos_on_click, receive_servo_arming_events, tick_cooldowns,
    ArmServo, Servo, ServoActivated,
};

use crate::asset_setup::primitives::PrimitiveResources;
use crate::character_control_systems::camera_controls::apply_mouse_camera_movement;

use self::guns::{fire_guns, Gun};

pub mod arms;
pub mod guns;
pub mod projectiles;
pub mod servo;

pub struct GunplayPlugin;

impl Plugin for GunplayPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ArmServo>()
            .add_event::<ServoActivated>()
            .add_event::<Recoil>();
        app.insert_resource(PrimitiveResources::default());

        app.register_type::<Servo>()
            .register_type::<Arm>()
            .register_type::<Gun>();

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
                do_should_activate,
                fire_guns,
                (do_arm_recoil, do_shake_recoil),
            )
                .chain(),
        );
    }
}
