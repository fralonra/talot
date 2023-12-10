use std::collections::HashSet;

use bevy::prelude::*;
use talot_core::{Lot, Stats, ER};

// --------- User Interface --------- //
#[derive(Component)]
pub(super) enum MenuButtonAction {
    Rebirth,
    Resume,
    Tombstone,
    BackToMainMenu,
}

#[derive(Component)]
pub(super) struct OnGameScreen;

#[derive(Component)]
pub(super) struct OnGameOverScreen;

#[derive(Component)]
pub(super) struct OnGameOverTombstoneScreen;

#[derive(Component)]
pub(super) struct OnGameSuspendScreen;

#[derive(Component, Default)]
pub(super) struct ScrollingList {
    pub position: f32,
}

#[derive(Component)]
pub(super) struct UiAgeLabel;

#[derive(Component)]
pub(super) struct UiAttrsPanel;

#[derive(Component)]
pub(super) struct UiTimelinePanel;

#[derive(Component)]
pub(super) struct UiERSprite;

#[derive(Component)]
pub(super) struct UiGameArea;

#[derive(Component)]
pub(super) struct UiPlayerStatIntuitionLabel;

#[derive(Component)]
pub(super) struct UiPlayerStatKnowledgeLabel;

#[derive(Component)]
pub(super) struct UiPlayerStatPhysicalLabel;

#[derive(Component)]
pub(super) struct UiPlayerStatSocialLabel;

// --------- Gameplay --------- //
#[derive(Component, Deref, DerefMut)]
pub(super) struct Age(pub f32);

#[derive(Component, Default, Deref, DerefMut)]
pub(super) struct Attributable(pub HashSet<u32>);

#[derive(Component, Deref, DerefMut)]
pub(super) struct CanHappen(pub bool);

#[derive(Component, Default, Deref, DerefMut)]
pub(super) struct EmotionalRating(pub ER);

#[derive(Component)]
pub(super) struct Player;

#[derive(Component, Default, Deref, DerefMut)]
pub(super) struct PlayerStat(pub Stats);

#[derive(Component, Deref, DerefMut)]
pub(super) struct Speed(pub f32);

#[derive(Component, Deref, DerefMut)]
pub(super) struct Trifle(pub Lot);
