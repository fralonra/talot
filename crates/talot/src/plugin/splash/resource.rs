use bevy::prelude::*;

#[derive(Deref, DerefMut, Resource)]
pub(super) struct AnimationTimer(pub Timer);
