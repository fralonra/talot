use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States)]
pub(super) enum MenuState {
    Main,
    Settings,
    #[default]
    Disabled,
}
