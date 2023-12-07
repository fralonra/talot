use bevy::prelude::*;

use crate::asset::GameAsset;

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
