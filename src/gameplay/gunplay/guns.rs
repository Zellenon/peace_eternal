use bevy::{
    ecs::component::Component,
    math::{Quat, Vec3},
    prelude::{Entity, Event, EventReader, EventWriter},
    reflect::Reflect,
};

#[derive(Component, Reflect, Clone, Debug, PartialEq)]
pub struct Gun;

#[derive(Event, Reflect, Clone, Debug, PartialEq)]
pub struct FireGun {
    pub entity: Entity,
    pub position: Vec3,
    pub orientation: Quat,
}

pub fn event_mirror_a<T: Event>(mut events: EventReader<T>, mut mirror: EventWriter<>)

pub fn propagate_to_guns(
    mut activations: EventReader<ServoActivated>,
    mut directed_activations: EventWriter<DirectedServoActivated>,
    servos: Query<&Children, With<Servo>>,
    barrels: Query<(Entity, &GlobalTransform), With<Barrel>>,
) {
    for ServoActivated(entity) in activations.read() {
        if let Ok(children) = servos.get(*entity) {
            let (barrel, position) = children
                .iter()
                .filter_map(|w| barrels.get(*w).ok())
                .next()
                .unwrap();
            let (_, rot, loc) = position.to_scale_rotation_translation();

            directed_activations.send(DirectedServoActivated {
                servo: *entity,
                barrel,
                location: loc,
                rotation: rot,
            });
        }
    }
}


// pub fn fire_guns(
//     mut servo_activations: EventReader<DirectedServoActivated>,
//     mut recoils: EventWriter<Recoil>,
//     mut audio_send: EventWriter<SpawnAudioBlip>,
//     mut flash_send: EventWriter<SpawnFlash>,
//     mut flare_send: EventWriter<SpawnMuzzleFlare>,
//     mut gunshots: EventWriter<FireGun>,
//     guns: Query<(&Children, &Parent), (With<Gun>, Without<Barrel>)>,
//     mut muzzle_flash_particles: Query<&mut EffectSpawner, With<MuzzleFlashFX>>,
//     barrels: Query<(&GlobalTransform, &Children), Without<Gun>>,
//     placeholder_audio: Res<PlaceholderAudio>,
// ) {
//     for ServoActivated(entity) in servo_activations.read() {
//         if let Ok((children, parent)) = guns.get(*entity) {
//             let (barrel, barrel_fx) = barrels.get(*children.iter().next().unwrap()).unwrap();
//             let (_, rot, loc) = barrel.to_scale_rotation_translation();

//             audio_send.send(SpawnAudioBlip {
//                 handle: placeholder_audio.rifle1.clone(),
//                 location: loc,
//                 volume: 1.0,
//                 stick_to: Some(parent.get()),
//             });

//             flash_send.send(SpawnFlash {
//                 location: loc,
//                 size: 0.3,
//             });

//             flare_send.send(SpawnMuzzleFlare {
//                 location: loc,
//                 size: 0.5,
//                 direction: rot,
//             });

//             recoils.send(Recoil {
//                 arm: parent.get(),
//                 strength: 1.,
//             });

//             barrel_fx.iter().for_each(|w| {
//                 let _ = muzzle_flash_particles.get_mut(*w).map(|mut w2| w2.reset());
//             });

//             gunshots.send(FireGun {
//                 entity: *entity,
//                 position: loc,
//                 orientation: rot,
//             });
//         }
//     }
// }
