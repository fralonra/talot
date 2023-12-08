use bevy::prelude::*;

// Timers
#[derive(Deref, DerefMut, Resource)]
pub(super) struct AgingTimer(pub Timer);

#[derive(Deref, DerefMut, Resource)]
pub(super) struct TrifleSpawnTimer(pub Timer);

// Game datas
#[derive(Default, Deref, DerefMut, Resource)]
pub(super) struct Attributes(pub Vec<String>);

#[derive(Default, Deref, DerefMut, Resource)]
pub(super) struct Bio(pub Vec<(f32, String, u32)>);
