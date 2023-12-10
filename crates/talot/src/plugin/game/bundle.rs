use bevy::prelude::*;

use crate::constant::TEXT_COLOR;

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
                        color: TEXT_COLOR,
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
