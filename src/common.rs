use bevy::prelude::*;

pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);

pub fn despawn_screen<T: Component>(mut commands: Commands, query_despawn: Query<Entity, With<T>>) {
    for entity in &query_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
