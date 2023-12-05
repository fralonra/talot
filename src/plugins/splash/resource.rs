use bevy::prelude::*;

#[derive(Deref, DerefMut, Resource)]
pub(super) struct SplashTimer(pub Timer);
