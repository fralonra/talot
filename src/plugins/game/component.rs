use bevy::prelude::*;

#[derive(Component)]
pub(super) enum TrifleType {
    Good,
    Bad,
}

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
#[derive(Component)]
pub(super) struct Age(pub u32);

#[derive(Component)]
pub(super) struct EmotionalImpactFactor(pub f32);

#[derive(Component)]
pub(super) struct EmotionalRating(pub f32);

#[derive(Component)]
pub(super) struct Player;

#[derive(Component, Default)]
pub(super) struct PlayerStat {
    pub intuition: f64,
    pub knowledge: f64,
    pub physical: f64,
    pub social: f64,
}

#[derive(Component)]
pub(super) struct Speed(pub f32);

#[derive(Component)]
pub(super) struct Trifle(pub TrifleType);
