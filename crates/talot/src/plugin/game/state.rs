use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States)]
pub enum InGameState {
    Playing,
    Suspend,
    Over,
    #[default]
    Disabled,
}
