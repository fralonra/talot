use bevy::prelude::*;

use crate::{common::despawn_screen, state::GameState};

use super::{
    component::{OnSplashScreen, SpriteLol, SpriteTot},
    resource::AnimationTimer,
};

pub(crate) struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), setup)
            .add_systems(
                Update,
                sprite_animate_system.run_if(in_state(GameState::Splash)),
            )
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let lol = asset_server.load("images/lol.png");
    let tot = asset_server.load("images/tot.png");

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
            parent.spawn((
                ImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Px(50.0),
                        ..default()
                    },
                    image: UiImage::new(lol),
                    ..default()
                },
                SpriteLol,
            ));

            parent.spawn((
                ImageBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Px(50.0),

                        ..default()
                    },
                    image: UiImage::new(tot),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                SpriteTot,
            ));

            parent.spawn(
                TextBundle::from_sections([TextSection::new(
                    "LOADING".to_owned(),
                    TextStyle {
                        font_size: 32.0,
                        color: Color::GOLD,
                        ..default()
                    },
                )])
                .with_style(Style {
                    margin: UiRect::top(Val::Px(100.0)),
                    ..default()
                }),
            );
        });

    commands.insert_resource(AnimationTimer(Timer::from_seconds(
        0.2,
        TimerMode::Repeating,
    )));
}

fn sprite_animate_system(
    (mut query_lol, mut query_tot): (
        Query<&mut Visibility, (With<SpriteLol>, Without<SpriteTot>)>,
        Query<&mut Visibility, (With<SpriteTot>, Without<SpriteLol>)>,
    ),
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
) {
    if timer.tick(time.delta()).finished() {
        let mut vis_lol = query_lol.single_mut();
        let mut vis_tot = query_tot.single_mut();

        if *vis_lol == Visibility::Hidden {
            *vis_lol = Visibility::Visible;
            *vis_tot = Visibility::Hidden;
        } else {
            *vis_lol = Visibility::Hidden;
            *vis_tot = Visibility::Visible;
        }
    }
}
