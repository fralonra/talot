use bevy::prelude::*;

use crate::constant::NORMAL_VOLUME;

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq, Resource)]
pub enum Difficulty {
    Farmer,
    Knight,
    DragonSlayer,
    Psychopath,
}

impl Difficulty {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Farmer => "Farmer",
            Self::Knight => "Knight",
            Self::DragonSlayer => "Dragon Slayer",
            Self::Psychopath => "Psychopath",
        }
    }

    pub fn player_speed_factor(&self) -> f32 {
        match self {
            Self::Farmer => 1.0,
            Self::Knight => 0.75,
            Self::DragonSlayer => 2.0,
            Self::Psychopath => 4.0,
        }
    }

    pub fn spawn_timer_factor(&self) -> f32 {
        match self {
            Self::Farmer => 1.0,
            Self::Knight => 0.5,
            Self::DragonSlayer => 0.5,
            Self::Psychopath => 0.2,
        }
    }
}

#[derive(Clone, Copy, Component, Debug, Eq, PartialEq, Resource)]
pub struct Volume(pub u32);

impl Volume {
    pub fn to_volume(&self) -> f32 {
        if NORMAL_VOLUME == 0 {
            return 0.0;
        }

        let step = 1.0 / NORMAL_VOLUME as f32;

        step * self.0 as f32
    }
}
