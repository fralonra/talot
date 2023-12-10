use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States)]
pub enum InGameState {
    Playing,
    Suspend,
    Over,
    #[default]
    Disabled,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States)]
pub enum OverState {
    Menu,
    Tombstone,
    #[default]
    Disabled,
}
