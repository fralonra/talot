mod asset;
mod common;
mod constant;
mod plugin;
mod resource;
mod state;

use asset::{AudioAssets, GameAsset, GameDataAssets, ImageAssets};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use constant::{WINDOW_HEIGHT, WINDOW_WIDTH};
use plugin::{GamePlugin, MenuPlugin, SplashPlugin};
use resource::Difficulty;
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
        .insert_resource(Difficulty::Knight)
        .add_state::<GameState>()
        .add_loading_state(LoadingState::new(GameState::Splash).continue_to_state(GameState::Menu))
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Splash)
        .add_collection_to_loading_state::<_, GameDataAssets>(GameState::Splash)
        .add_collection_to_loading_state::<_, ImageAssets>(GameState::Splash)
        .add_systems(Startup, setup)
        .add_plugins((SplashPlugin, MenuPlugin, GamePlugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
