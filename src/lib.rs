mod common;
mod config;
mod core;
mod plugins;

use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use config::{WINDOW_HEIGHT, WINDOW_WIDTH};

use crate::core::GameAsset;

use self::{
    common::*,
    plugins::{GamePlugin, MenuPlugin, SplashPlugin},
};

pub fn run() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            TomlAssetPlugin::<GameAsset>::new(&["asset.toml"]),
        ))
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((SplashPlugin, MenuPlugin, GamePlugin))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let game_asset = GameAssetHandle(asset_server.load("core.asset.toml"));
    commands.insert_resource(game_asset);
}
