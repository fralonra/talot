use bevy::prelude::*;

use crate::{common::despawn_screen, state::GameState};

use super::{component::OnSplashScreen, resource::SplashTimer};

pub(crate) struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), setup)
            .add_systems(Update, countdown_system.run_if(in_state(GameState::Splash)))
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("branding/icon.png");

    // Logo
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(200.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        });

    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

fn countdown_system(
    time: Res<Time>,
    (mut game_state, mut timer): (ResMut<NextState<GameState>>, ResMut<SplashTimer>),
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}
