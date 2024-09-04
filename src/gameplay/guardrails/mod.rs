use bevy::{
    app::{Plugin, Update},
    prelude::{Commands, DespawnRecursiveExt, Entity, Query, Transform},
    reflect::Reflect,
};

#[derive(Reflect, Clone, Debug, PartialEq)]
pub struct GuardrailsPlugin;

impl Plugin for GuardrailsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, out_of_bounds_killer);
    }
}

pub(super) fn out_of_bounds_killer(mut commands: Commands, entities: Query<(Entity, &Transform)>) {
    for (entity, position) in entities.iter() {
        if [
            position.translation.x.abs(),
            position.translation.y.abs(),
            position.translation.z.abs(),
        ]
        .iter()
        .any(|w| w > &1000.)
        {
            println!("Had to despawn out-of-bounds entity");
            commands.get_entity(entity).map(|w| w.despawn_recursive());
        }
    }
}
