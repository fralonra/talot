mod asset;
mod common;
mod constant;
mod plugin;
mod resource;
mod state;

use asset::GameAsset;
use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use constant::{WINDOW_HEIGHT, WINDOW_WIDTH};
use plugin::{GamePlugin, MenuPlugin, SplashPlugin};
use resource::{DisplayQuality, GameAssetHandle, Volume};
use state::GameState;

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
            JsonAssetPlugin::<GameAsset>::new(&["asset.json"]),
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

    let game_asset = GameAssetHandle(asset_server.load("core.asset.json"));
    commands.insert_resource(game_asset);
}
