use bevy::app::{Plugin, Update};
use bevy::prelude::IntoSystemConfigs;

use crate::character_control_systems::camera_controls::apply_mouse_camera_movement;
use crate::util::primitives::Primitive_Resources;

use self::arms::{
    do_should_activate, player_arms_on_click, receive_arm_activation_events, tick_cooldowns,
    update_arm_position, ActivateArm, Arm, Servo, ServoActivated,
};
use self::guns::{fire_guns, Gun};

pub mod arms;
pub mod guns;

pub struct GunplayPlugin;

impl Plugin for GunplayPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<ActivateArm>().add_event::<ServoActivated>();
        app.insert_resource(Primitive_Resources::default());

        app.register_type::<Servo>()
            .register_type::<Arm>()
            .register_type::<Gun>();

        app.add_systems(
            Update,
            update_arm_position.after(apply_mouse_camera_movement),
        );

        app.add_systems(
            Update,
            player_arms_on_click.before(receive_arm_activation_events),
        )
        .add_systems(
            Update,
            (tick_cooldowns, receive_arm_activation_events).before(do_should_activate),
        )
        .add_systems(Update, do_should_activate);

        app.add_systems(Update, fire_guns.after(do_should_activate));
    }
}
