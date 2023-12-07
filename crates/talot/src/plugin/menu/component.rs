use bevy::prelude::*;

#[derive(Component)]
pub(super) enum MenuButtonAction {
    Play,
    Settings,
    BackToMainMenu,
    Quit,
}

#[derive(Component)]
pub(super) struct OnMainMenuScreen;

#[derive(Component)]
pub(super) struct OnSettingsMenuScreen;

#[derive(Component)]
pub(super) struct SelectedOption;
