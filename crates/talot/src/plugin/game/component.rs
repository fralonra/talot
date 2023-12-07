use bevy::{prelude::*, utils::HashSet};
use talot_core::{Lot, Stats};

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
pub(super) struct Attributable(pub HashSet<u32>);

impl Attributable {
    pub fn to_vec(&self) -> Vec<u32> {
        self.0.iter().cloned().collect()
    }
}

#[derive(Component)]
pub(super) struct EmotionalImpactFactor(pub f32);

#[derive(Component)]
pub(super) struct EmotionalRating(pub f32);

#[derive(Component)]
pub(super) struct Player;

#[derive(Component, Default, Deref, DerefMut)]
pub(super) struct PlayerStat(pub Stats);

#[derive(Component)]
pub(super) struct Speed(pub f32);

#[derive(Component)]
pub(super) struct Trifle {
    pub lot: Lot,
    pub can_happend: bool,
}

impl Trifle {
    pub fn new(lot: Lot) -> Self {
        Self {
            lot,
            can_happend: false,
        }
    }
}
