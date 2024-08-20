use std::time::Duration;

use bevy::{
    prelude::{
        Commands, Component, DespawnRecursiveExt, Entity, Event, EventReader, EventWriter, Query,
        Res, SystemSet, With,
    },
    reflect::Reflect,
    time::{Time, Timer, TimerMode},
};

#[derive(Component, Debug, Reflect)]
pub struct Deathmarker;

#[derive(Component, Debug, Reflect)]
pub struct DelayedDeathmarker;

#[derive(Event, Debug, Reflect)]
pub struct Destroy(Entity);

#[derive(Component)]
pub struct Lifespan(pub Timer);

impl Lifespan {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, TimerMode::Once))
    }
}

impl Default for Lifespan {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(500), TimerMode::Once))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, SystemSet, Reflect)]
pub struct DestructionSet;

pub(super) fn destroy_death_markers(
    mut events: EventWriter<Destroy>,
    markers: Query<Entity, With<Deathmarker>>,
) {
    for marker in markers.iter() {
        events.send(Destroy(marker));
    }
}

pub(super) fn delayed_death_markers(
    mut events: EventWriter<Destroy>,
    markers: Query<Entity, With<DelayedDeathmarker>>,
) {
    for marker in markers.iter() {
        events.send(Destroy(marker));
    }
}

pub(super) fn despawn_destroyed_entities(mut events: EventReader<Destroy>, mut commands: Commands) {
    events.read().for_each(|event| {
        commands.get_entity(event.0).map(|w| w.despawn_recursive());
    });
}

pub(super) fn tick_lifespans(time: Res<Time>, mut lifespans: Query<&mut Lifespan>) {
    for mut life in lifespans.iter_mut() {
        life.0.tick(time.delta());
    }
}

pub(super) fn end_lifespan(
    mut events: EventWriter<Destroy>,
    lifespans: Query<(Entity, &Lifespan)>,
) {
    for (e, Lifespan(timer)) in lifespans.iter() {
        if timer.finished() {
            events.send(Destroy(e));
        }
    }
}
