use bevy::prelude::*;

use crate::config::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub(super) const GAME_AREA_BORDER_WIDTH: f32 = 15.0;
pub(super) const GAME_AREA_HEIGHT: f32 = WINDOW_HEIGHT - 40.0;
pub(super) const GAME_AREA_WIDTH: f32 =
    WINDOW_WIDTH - PANEL_WIDTH * 2.0 - GAME_AREA_BORDER_WIDTH * 2.0 - 40.0;

pub(super) const PANEL_BACKGROUND_COLOR: Color = Color::rgb(0.65, 0.65, 0.65);
pub(super) const PANEL_WIDTH: f32 = 200.0;

pub(super) const PLAYER_SIZE: Vec3 = Vec3::new(25.0, 25.0, 0.0);

pub(super) const TRIFLE_HEIGHT: f32 = 10.0;
