use bevy::prelude::*;

use super::component::{Speed, Trifle};

#[derive(Bundle)]
pub(super) struct StatBundle {
    text: TextBundle,
}

impl StatBundle {
    pub fn new(label: &str) -> Self {
        Self {
            text: TextBundle::from_sections([
                TextSection::new(
                    format!("{}: ", label),
                    TextStyle {
                        font_size: 30.0,
                        ..default()
                    },
                ),
                TextSection::from_style(TextStyle {
                    font_size: 30.0,
                    color: Color::GOLD,
                    ..default()
                }),
            ]),
        }
    }
}

#[derive(Bundle)]
pub(super) struct TrifleBundle {
    pub trifle: Trifle,
    pub speed: Speed,

    pub sprite: SpriteBundle,
}
