use bevy::{
    prelude::{
        Commands, Component, DespawnRecursiveExt, Entity, Event, EventReader, EventWriter, Query,
        With,
    },
    reflect::Reflect,
};

#[derive(Component, Debug, Reflect)]
pub struct Deathmarker;

#[derive(Event, Debug, Reflect)]
pub struct Destroy(Entity);

pub(super) fn kill_death_markers(
    mut events: EventWriter<Destroy>,
    markers: Query<Entity, With<Deathmarker>>,
) {
    for marker in markers.iter() {
        events.send(Destroy(marker));
    }
}

pub(super) fn destroy(mut events: EventReader<Destroy>, mut commands: Commands) {
    for event in events.read() {
        commands.get_entity(event.0).unwrap().despawn_recursive()
    }
}
