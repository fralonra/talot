use bevy::prelude::*;

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
    sprite: SpriteBundle,
}

impl Default for TrifleBundle {
    fn default() -> Self {
        Self {
            sprite: SpriteBundle::default(),
        }
    }
}

impl TrifleBundle {
    pub fn new(sprite: SpriteBundle) -> Self {
        Self {
            sprite,
            ..default()
        }
    }
}
