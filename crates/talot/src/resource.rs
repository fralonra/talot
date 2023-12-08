use bevy::prelude::*;

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq, Resource)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq, Resource)]
pub struct Volume(pub u32);
