use bevy::prelude::*;
use serde::Deserialize;
use talot_core::Engine;

#[derive(Asset, Debug, Deserialize, Deref, DerefMut, TypePath)]
pub struct GameAsset(pub Engine);
