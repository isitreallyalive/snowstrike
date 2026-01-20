use bevy::prelude::*;

pub mod button;

/// Despawn all entities with the given marker
pub fn clear<M: Component>(mut commands: Commands, query: Query<Entity, With<M>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
