use bevy::{app::AppExit, audio, prelude::*};

use crate::{
    asset::AudioAssets,
    common::despawn_screen,
    constant::{
        HOVERED_BUTTON_COLOR, HOVERED_PRESSED_BUTTON_COLOR, MENU_BACKGROUND_COLOR,
        NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR, TEXT_COLOR,
    },
    resource::{Difficulty, Volume},
    state::GameState,
};

use super::{
    component::{MenuButtonAction, OnMainMenuScreen, OnSettingsMenuScreen, SelectedOption},
    state::MenuState,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            // OnEnter
            .add_systems(OnEnter(GameState::Menu), setup)
            .add_systems(OnEnter(MenuState::Main), setup_main_menu)
            .add_systems(OnEnter(MenuState::Settings), setup_settings_menu)
            // Update
            .add_systems(
                Update,
                (
                    setting_button_system::<Difficulty>,
                    setting_button_system::<Volume>,
                )
                    .run_if(in_state(MenuState::Settings)),
            )
            .add_systems(
                Update,
                (button_system, menu_action_system).run_if(in_state(GameState::Menu)),
            )
            .add_systems(
                Update,
                (
                    setting_changed_system::<Difficulty>,
                    setting_changed_system::<Volume>,
                )
                    .run_if(in_state(MenuState::Settings)),
            )
            // OnExit
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            .add_systems(
                OnExit(MenuState::Settings),
                despawn_screen::<OnSettingsMenuScreen>,
            );
    }
}

fn setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn setup_main_menu(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(350.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(15.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 30.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: MENU_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "How are you today",
                            TextStyle {
                                font_size: 50.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(30.0)),
                            ..default()
                        }),
                    );

                    // Play
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            MenuButtonAction::Play,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Start A New Life",
                                button_text_style.clone(),
                            ));
                        });

                    // Settings
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            MenuButtonAction::Settings,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Want To Change Sth",
                                button_text_style.clone(),
                            ));
                        });

                    // Quit
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Bye",
                                TextStyle {
                                    font_size: 40.0,
                                    ..button_text_style.clone()
                                },
                            ));
                        });
                });
        });
}

fn setup_settings_menu(
    mut commands: Commands,
    (difficulty, volume): (Res<Difficulty>, Res<Volume>),
) {
    let button_style = Style {
        margin: UiRect::all(Val::Px(20.0)),
        padding: UiRect::px(16.0, 16.0, 10.0, 10.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnSettingsMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: MENU_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Difficulty
                    parent.spawn(
                        TextBundle::from_section("Difficulty", button_text_style.clone())
                            .with_style(Style {
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            }),
                    );

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            for difficulty_setting in [
                                Difficulty::Farmer,
                                Difficulty::Knight,
                                Difficulty::DragonSlayer,
                                Difficulty::Psychopath,
                            ] {
                                let mut entity = parent.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            margin: UiRect::all(Val::Px(10.0)),
                                            ..button_style.clone()
                                        },
                                        background_color: NORMAL_BUTTON_COLOR.into(),
                                        ..default()
                                    },
                                    difficulty_setting,
                                ));

                                entity.with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        difficulty_setting.label(),
                                        TextStyle {
                                            font_size: 20.0,
                                            ..button_text_style.clone()
                                        },
                                    ));
                                });

                                if *difficulty == difficulty_setting {
                                    entity.insert(SelectedOption);
                                }
                            }
                        });

                    // Sounds
                    parent.spawn(
                        TextBundle::from_section("Sounds", button_text_style.clone()).with_style(
                            Style {
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                        ),
                    );

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("0", button_text_style.clone()));

                            for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                let mut entity = parent.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            margin: UiRect::all(Val::Px(10.0)),
                                            ..button_style.clone()
                                        },
                                        background_color: NORMAL_BUTTON_COLOR.into(),
                                        ..default()
                                    },
                                    Volume(volume_setting),
                                ));

                                if *volume == Volume(volume_setting) {
                                    entity.insert(SelectedOption);
                                }
                            }

                            parent
                                .spawn(TextBundle::from_section("100", button_text_style.clone()));
                        });

                    // Back
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            MenuButtonAction::BackToMainMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Back", button_text_style));
                        });
                });
        });
}

fn button_system(
    mut query_interaction: Query<
        (&mut BackgroundColor, &Interaction, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (mut color, interaction, selected) in &mut query_interaction {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON_COLOR.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON_COLOR.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON_COLOR.into(),
            (Interaction::None, None) => NORMAL_BUTTON_COLOR.into(),
        }
    }
}

fn menu_action_system(
    query_interaction: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    (mut game_state, mut menu_state): (ResMut<NextState<GameState>>, ResMut<NextState<MenuState>>),
) {
    for (interaction, menu_button_action) in &query_interaction {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => {
                    game_state.set(GameState::Game);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
            }
        }
    }
}

fn setting_button_system<T: Resource + Component + PartialEq + Copy>(
    mut commands: Commands,
    (query_interaction, mut query_selected): (
        Query<(Entity, &Interaction, &T), (Changed<Interaction>, With<Button>)>,
        Query<(Entity, &mut BackgroundColor), (With<SelectedOption>, With<T>)>,
    ),
    mut setting: ResMut<T>,
) {
    for (entity, interaction, button_setting) in &query_interaction {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            for (previous_button, mut previous_color) in query_selected.iter_mut() {
                *previous_color = NORMAL_BUTTON_COLOR.into();

                commands.entity(previous_button).remove::<SelectedOption>();
                commands.entity(entity).insert(SelectedOption);

                *setting = *button_setting;
            }
        }
    }
}

fn setting_changed_system<T: Resource>(
    mut commands: Commands,
    (audio_assets, res, volume): (Res<AudioAssets>, Res<T>, Res<Volume>),
) {
    if res.is_changed() && !res.is_added() {
        commands.spawn(AudioBundle {
            source: audio_assets.dong.clone(),
            settings: PlaybackSettings {
                volume: audio::Volume::new_relative(volume.to_volume()),
                ..default()
            },
        });
    }
}
