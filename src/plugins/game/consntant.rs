use bevy::prelude::*;

use crate::config::WINDOW_WIDTH;

pub(super) const GAME_AREA_BORDER_COLOR: Color = Color::rgb(0.65, 0.65, 0.65);
pub(super) const GAME_AREA_BORDER_WIDTH: f32 = 5.0;
pub(super) const GAME_AREA_WIDTH: f32 =
    WINDOW_WIDTH - 2.0 * PANEL_WIDTH - 2.0 * GAME_AREA_BORDER_WIDTH - 40.0;

pub(super) const PANEL_BACKGROUND_COLOR: Color = Color::rgb(0.65, 0.65, 0.65);
pub(super) const PANEL_WIDTH: f32 = 200.0;

pub(super) const PLAYER_SIZE: Vec3 = Vec3::new(50.0, 50.0, 0.0);
