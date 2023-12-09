use bevy::prelude::*;

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq, Resource)]
pub enum Difficulty {
    Farmer,
    Knight,
    DragonSlayer,
    Psychopath,
}

impl Difficulty {
    pub fn label(self) -> &'static str {
        match self {
            Self::Farmer => "Farmer",
            Self::Knight => "Knight",
            Self::DragonSlayer => "Dragon Slayer",
            Self::Psychopath => "Psychopath",
        }
    }

    pub fn player_speed_factor(self) -> f32 {
        match self {
            Self::Farmer => 1.0,
            Self::Knight => 0.75,
            Self::DragonSlayer => 2.0,
            Self::Psychopath => 4.0,
        }
    }

    pub fn spawn_timer_factor(self) -> f32 {
        match self {
            Self::Farmer => 1.0,
            Self::Knight => 0.5,
            Self::DragonSlayer => 0.5,
            Self::Psychopath => 0.2,
        }
    }
}
