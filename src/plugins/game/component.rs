use bevy::prelude::*;

use crate::core::Lot;

// --------- User Interface --------- //
#[derive(Component)]
pub(super) struct OnGameScreen;

#[derive(Component)]
pub(super) struct UiAgeLabel;

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

#[derive(Component)]
pub(super) struct EmotionalImpactFactor(pub f32);

#[derive(Component)]
pub(super) struct EmotionalRating(pub f32);

#[derive(Component)]
pub(super) struct Player;

#[derive(Component, Default)]
pub(super) struct PlayerStat {
    pub intuition: f32,
    pub knowledge: f32,
    pub physical: f32,
    pub social: f32,
}

#[derive(Component)]
pub(super) struct Speed(pub f32);

#[derive(Component)]
pub(super) struct Trifle(pub Lot);
