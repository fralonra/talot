mod common;
mod plugins;

use bevy::prelude::*;

use self::{
    common::*,
    plugins::{GamePlugin, MenuPlugin, SplashPlugin},
};

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((SplashPlugin, MenuPlugin, GamePlugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
