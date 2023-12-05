use bevy::prelude::*;

#[derive(Deref, DerefMut, Resource)]
pub(super) struct TrifleSpawnTimer(pub Timer);
