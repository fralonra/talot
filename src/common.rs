use bevy::prelude::*;

use crate::core::GameAsset;

pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq, Resource)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

#[derive(Resource)]
pub struct GameAssetHandle(pub Handle<GameAsset>);

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq, Resource)]
pub struct Volume(pub u32);

pub fn despawn_screen<T: Component>(mut commands: Commands, query_despawn: Query<Entity, With<T>>) {
    for entity in &query_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
