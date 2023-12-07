use bevy::prelude::*;

#[derive(Deref, DerefMut, Resource)]
pub(super) struct AgingTimer(pub Timer);

#[derive(Deref, DerefMut, Resource)]
pub(super) struct TrifleSpawnTimer(pub Timer);
