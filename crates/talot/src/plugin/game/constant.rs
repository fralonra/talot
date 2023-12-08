use bevy::prelude::*;

use crate::constant::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub(super) const GAME_AREA_HEIGHT: f32 =
    WINDOW_HEIGHT - PANEL_BOTTOM_HEIGHT * 2.0 - GAME_AREA_MARGIN * 2.0;
pub(super) const GAME_AREA_MARGIN: f32 = 40.0;
pub(super) const GAME_AREA_WIDTH: f32 =
    WINDOW_WIDTH - PANEL_LEFT_WIDTH - PANEL_RIGHT_WIDTH - GAME_AREA_MARGIN * 2.0;

pub(super) const PANEL_BACKGROUND_COLOR: Color = Color::rgb(0.65, 0.65, 0.65);
pub(super) const PANEL_BOTTOM_HEIGHT: f32 = 100.0;
pub(super) const PANEL_LEFT_WIDTH: f32 = 250.0;
pub(super) const PANEL_RIGHT_WIDTH: f32 = 250.0;

pub(super) const PLAYER_SIZE: Vec2 = Vec2::new(25.0, 25.0);

pub(super) const TRIFLE_HEIGHT: f32 = 10.0;
pub(super) const TRIFLE_LABEL_FONT_SIZE: f32 = 16.0;
