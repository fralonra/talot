use bevy::prelude::*;

pub fn despawn_screen<T: Component>(mut commands: Commands, query_despawn: Query<Entity, With<T>>) {
    for entity in &query_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
