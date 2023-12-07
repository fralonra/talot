use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}
