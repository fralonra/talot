use bevy::{
    audio,
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use rand::Rng;
use rand_distr::{Distribution, Normal};
use talot_core::{Attribute, QueryInfo, RespInfo};

use crate::{
    asset::{AudioAssets, GameAsset, GameDataAssets, ImageAssets},
    common::despawn_screen,
    constant::{
        HOVERED_BUTTON_COLOR, MENU_BACKGROUND_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR,
        TEXT_COLOR,
    },
    resource::{Difficulty, Volume},
    state::GameState,
};

use super::{
    bundle::StatBundle,
    component::{
        Age, Attributable, CanHappen, EmotionalRating, MenuButtonAction, OnGameOverScreen,
        OnGameScreen, OnGameSuspendScreen, Player, PlayerStat, ScrollingList, Speed, Trifle,
        UiAgeLabel, UiAttrsPanel, UiBioPanel, UiERSprite, UiGameArea, UiPlayerStatIntuitionLabel,
        UiPlayerStatKnowledgeLabel, UiPlayerStatPhysicalLabel, UiPlayerStatSocialLabel,
    },
    constant::{
        ER_CAPACITY, ER_SPRITE_GAP, ER_SPRITE_SIZE, GAME_AREA_HEIGHT, GAME_AREA_MARGIN,
        GAME_AREA_WIDTH, MODAL_BACKGROUND_COLOR, PANEL_BACKGROUND_COLOR, PANEL_BOTTOM_HEIGHT,
        PANEL_LEFT_WIDTH, PANEL_RIGHT_WIDTH, PLAYER_SIZE, TRIFLE_HEIGHT, TRIFLE_LABEL_FONT_SIZE,
    },
    resource::{AgingTimer, Attributes, Bio, TrifleSpawnTimer},
    state::InGameState,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<InGameState>()
            // OnEnter
            .add_systems(OnEnter(GameState::Game), setup)
            .add_systems(OnEnter(InGameState::Playing), setup_playing)
            .add_systems(OnEnter(InGameState::Suspend), setup_suspend)
            .add_systems(OnEnter(InGameState::Over), setup_over)
            // Update
            .add_systems(
                Update,
                (check_over_system).run_if(in_state(InGameState::Playing)),
            )
            .add_systems(
                Update,
                (keyboard_system, mouse_system).run_if(in_state(InGameState::Playing)),
            )
            .add_systems(
                Update,
                (player_aging_system, player_moving_system).run_if(in_state(InGameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    trifle_spawn_system,
                    trifle_update_system,
                    trifle_handle_system,
                    trifle_miss_system,
                )
                    .run_if(in_state(InGameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    er_bar_system,
                    text_age_system,
                    text_attrs_system,
                    text_bio_system,
                    text_stat_int_system,
                    text_stat_kno_system,
                    text_stat_phy_system,
                    text_stat_soc_system,
                )
                    .run_if(in_state(InGameState::Playing)),
            )
            .add_systems(
                Update,
                (button_system, menu_action_system)
                    .run_if(in_state(GameState::Game))
                    .run_if(not(in_state(InGameState::Playing))),
            )
            // OnExit
            .add_systems(
                OnExit(InGameState::Suspend),
                despawn_screen::<OnGameSuspendScreen>,
            )
            .add_systems(OnExit(InGameState::Over), despawn_screen::<OnGameScreen>)
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

fn setup(mut in_game_state: ResMut<NextState<InGameState>>) {
    in_game_state.set(InGameState::Playing);
}

fn setup_over(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
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
                background_color: MODAL_BACKGROUND_COLOR.into(),
                ..default()
            },
            OnGameScreen,
            OnGameOverScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: MENU_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "You are DEPRESSED",
                            TextStyle {
                                font_size: 50.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(40.0)),
                            ..default()
                        }),
                    );

                    // Rebirth
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            MenuButtonAction::Rebirth,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Rebirth",
                                button_text_style.clone(),
                            ));
                        });

                    // Back To Menu
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
                            parent
                                .spawn(TextBundle::from_section("Back To Menu", button_text_style));
                        });
                });
        });
}

fn setup_playing(
    mut commands: Commands,
    query_player: Query<&Player>,
    (game_assets, difficulty, asset_handles, image_assets): (
        Res<Assets<GameAsset>>,
        Res<Difficulty>,
        Res<GameDataAssets>,
        Res<ImageAssets>,
    ),
) {
    if !query_player.is_empty() {
        return;
    }

    let game_asset = game_assets.get(&asset_handles.core).unwrap();
    game_asset.reset();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            OnGameScreen,
        ))
        .with_children(|parent| {
            // Left Panel
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(PANEL_LEFT_WIDTH),
                        height: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(5.0)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: PANEL_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Difficulty
                    parent.spawn(
                        TextBundle::from_section(
                            format!("Difficulty: {}", difficulty.label()),
                            TextStyle {
                                font_size: 16.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::bottom(Val::Px(12.0)),
                            ..default()
                        }),
                    );

                    // Age
                    parent.spawn((
                        TextBundle::from_sections([
                            TextSection::new(
                                "Age: ",
                                TextStyle {
                                    font_size: 30.0,
                                    color: TEXT_COLOR,
                                    ..default()
                                },
                            ),
                            TextSection::from_style(TextStyle {
                                font_size: 30.0,
                                color: Color::BLUE,
                                ..default()
                            }),
                        ])
                        .with_style(Style {
                            margin: UiRect::bottom(Val::Px(12.0)),
                            ..default()
                        }),
                        UiAgeLabel,
                    ));

                    // Physical Stat
                    parent.spawn((StatBundle::new("PHY"), UiPlayerStatPhysicalLabel));

                    // Intuition Stat
                    parent.spawn((StatBundle::new("INT"), UiPlayerStatIntuitionLabel));

                    // Knowledge Stat
                    parent.spawn((StatBundle::new("KNO"), UiPlayerStatKnowledgeLabel));

                    // Social Stat
                    parent.spawn((StatBundle::new("SOC"), UiPlayerStatSocialLabel));

                    parent.spawn((NodeBundle::default(), UiAttrsPanel));
                });

            // Center
            parent
                .spawn(NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(20.0)),
                        align_self: AlignSelf::FlexStart,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Game Tips
                    parent.spawn(
                        TextBundle::from_section(
                            "Move your character using arrow keys.",
                            TextStyle {
                                font_size: 20.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::bottom(Val::Px(8.0)),
                            ..default()
                        }),
                    );

                    parent.spawn(
                        TextBundle::from_section(
                            "Collect lol, avoid tot.",
                            TextStyle {
                                font_size: 20.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::bottom(Val::Px(8.0)),
                            ..default()
                        }),
                    );

                    parent.spawn(
                        TextBundle::from_section(
                            "Find your way to a remarkable life.",
                            TextStyle {
                                font_size: 20.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::bottom(Val::Px(8.0)),
                            ..default()
                        }),
                    );

                    parent.spawn(
                        TextBundle::from_section(
                            "Don't get depressed.",
                            TextStyle {
                                font_size: 20.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::bottom(Val::Px(8.0)),
                            ..default()
                        }),
                    );
                });

            // Right Panel
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(PANEL_RIGHT_WIDTH),
                        height: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(5.0)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: PANEL_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Bio
                    parent.spawn(TextBundle::from_section(
                        "Bio",
                        TextStyle {
                            font_size: 30.0,
                            color: TEXT_COLOR,
                            ..default()
                        },
                    ));

                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                align_self: AlignSelf::Stretch,
                                flex_direction: FlexDirection::Column,
                                overflow: Overflow::clip_y(),
                                ..default()
                            },
                            ..default()
                        },
                        UiBioPanel,
                    ));
                });
        });

    // Game Area
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(GAME_AREA_WIDTH, GAME_AREA_HEIGHT)),
                ..default()
            },
            texture: image_assets.game_area.clone(),
            ..default()
        },
        OnGameScreen,
        UiGameArea,
    ));

    // ER Bar
    for i in 0..ER_CAPACITY as usize {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(ER_SPRITE_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    (i as f32 + 0.5 - ER_CAPACITY * 0.5) * (ER_SPRITE_GAP + ER_SPRITE_SIZE),
                    -GAME_AREA_HEIGHT * 0.5 - GAME_AREA_MARGIN - PANEL_BOTTOM_HEIGHT * 0.5,
                    0.0,
                )),
                texture: image_assets.empty_ef.clone(),
                ..default()
            },
            OnGameScreen,
            UiERSprite,
        ));
    }

    // Player
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                0.0,
                (PLAYER_SIZE.y - GAME_AREA_HEIGHT) * 0.5,
                1.0,
            )),
            texture: image_assets.player.clone(),
            ..default()
        },
        Player,
        PlayerStat::default(),
        Age(0.0),
        Attributable::default(),
        EmotionalRating::default(),
        Speed(200.0 * difficulty.player_speed_factor()),
        OnGameScreen,
    ));

    commands.insert_resource(AgingTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
    commands.insert_resource(TrifleSpawnTimer(Timer::from_seconds(
        1.0 * difficulty.spawn_timer_factor(),
        TimerMode::Repeating,
    )));

    commands.insert_resource(Attributes::default());
    commands.insert_resource(Bio::default());
}

fn setup_suspend(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
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
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: MODAL_BACKGROUND_COLOR.into(),
                ..default()
            },
            OnGameScreen,
            OnGameSuspendScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: MENU_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "You slow down your thoughts",
                            TextStyle {
                                font_size: 40.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(40.0)),
                            ..default()
                        }),
                    );

                    // Resume
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            },
                            MenuButtonAction::Resume,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Resume",
                                button_text_style.clone(),
                            ));
                        });

                    // Back To Menu
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
                            parent
                                .spawn(TextBundle::from_section("Back To Menu", button_text_style));
                        });
                });
        });
}

fn button_system(
    mut query_interaction: Query<
        (&mut BackgroundColor, &Interaction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (mut color, interaction) in &mut query_interaction {
        *color = match *interaction {
            Interaction::Pressed => PRESSED_BUTTON_COLOR.into(),
            Interaction::Hovered => HOVERED_BUTTON_COLOR.into(),
            Interaction::None => NORMAL_BUTTON_COLOR.into(),
        }
    }
}

fn check_over_system(
    query_er: Query<&EmotionalRating, (Changed<EmotionalRating>, With<Player>)>,
    mut in_game_state: ResMut<NextState<InGameState>>,
) {
    if let Ok(er) = query_er.get_single() {
        if er.tot >= ER_CAPACITY {
            in_game_state.set(InGameState::Over);
        }
    }
}

fn er_bar_system(
    (query_er, mut query_sprites): (
        Query<&EmotionalRating, (Changed<EmotionalRating>, With<Player>)>,
        Query<&mut Handle<Image>, With<UiERSprite>>,
    ),
    image_assets: Res<ImageAssets>,
) {
    if let Ok(er) = query_er.get_single() {
        let lol = er.lol as usize;
        let tot = er.tot as usize;

        for (i, mut handle) in &mut query_sprites.iter_mut().enumerate() {
            if ER_CAPACITY as usize - i <= lol {
                *handle = image_assets.lol.clone();
            } else {
                *handle = image_assets.empty_ef.clone();
            }
        }

        for (i, mut handle) in &mut query_sprites.iter_mut().enumerate() {
            if i < tot {
                *handle = image_assets.tot.clone();
            }
        }
    }
}

fn keyboard_system(input: Res<Input<KeyCode>>, mut in_game_state: ResMut<NextState<InGameState>>) {
    if input.pressed(KeyCode::Escape) {
        in_game_state.set(InGameState::Suspend);
    }
}

fn menu_action_system(
    query_interaction: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    (mut game_state, mut ingame_state): (
        ResMut<NextState<GameState>>,
        ResMut<NextState<InGameState>>,
    ),
) {
    for (interaction, menu_button_action) in &query_interaction {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Rebirth => {
                    ingame_state.set(InGameState::Playing);
                }
                MenuButtonAction::Resume => {
                    ingame_state.set(InGameState::Playing);
                }
                MenuButtonAction::BackToMainMenu => {
                    game_state.set(GameState::Menu);
                    ingame_state.set(InGameState::Disabled);
                }
            }
        }
    }
}

fn mouse_system(
    (mut query_list, query_node): (
        Query<(&Node, &Parent, &mut ScrollingList, &mut Style)>,
        Query<&Node>,
    ),
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (list_node, parent, mut scrolling_list, mut style) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.0);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.0,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.0);
            style.top = Val::Px(scrolling_list.position);
        }
    }
}

fn player_aging_system(
    mut query_player: Query<(&mut Age, &mut EmotionalRating), With<Player>>,
    time: Res<Time>,
    mut timer: ResMut<AgingTimer>,
) {
    let (mut age, mut er) = query_player.single_mut();

    if timer.tick(time.delta()).finished() {
        age.0 += 1.0;

        er.lol = 0.0_f32.max(er.lol - 1.0);
    }
}

fn player_moving_system(
    mut query_player: Query<(&Speed, &mut Sprite, &mut Transform), With<Player>>,
    (input, time): (Res<Input<KeyCode>>, Res<Time>),
) {
    let (speed, mut sprite, mut transform) = query_player.single_mut();

    let direction = if input.pressed(KeyCode::Left) {
        -1.0
    } else if input.pressed(KeyCode::Right) {
        1.0
    } else {
        0.0
    };

    let new_position = transform.translation.x + direction * speed.0 * time.delta_seconds();

    let left_bound = (PLAYER_SIZE.x - GAME_AREA_WIDTH) * 0.5;
    let right_bound = (GAME_AREA_WIDTH - PLAYER_SIZE.x) * 0.5;

    transform.translation.x = new_position.clamp(left_bound, right_bound);
    if direction != 0.0 {
        sprite.flip_x = direction > 0.0;
    }
}

fn text_age_system(
    (query_age, mut query_text): (
        Query<&Age, (Changed<Age>, With<Player>)>,
        Query<&mut Text, With<UiAgeLabel>>,
    ),
) {
    if let Ok(age) = query_age.get_single() {
        for mut text in &mut query_text {
            text.sections[1].value = format!("{:.0}", age.0);
        }
    }
}

fn text_attrs_system(
    mut commands: Commands,
    query_entity: Query<Entity, With<UiAttrsPanel>>,
    attrs: Res<Attributes>,
) {
    if attrs.is_changed() {
        let entity = query_entity.single();

        commands.entity(entity).despawn_descendants();

        commands
            .entity(entity)
            .insert(NodeBundle {
                style: Style {
                    padding: UiRect::top(Val::Px(20.0)),
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                for attr in &**attrs {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                border: UiRect::all(Val::Px(1.0)),
                                margin: UiRect::px(0.0, 8.0, 8.0, 0.0),
                                padding: UiRect::px(8.0, 8.0, 4.0, 4.0),
                                ..default()
                            },
                            border_color: Color::ORANGE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                attr,
                                TextStyle {
                                    font_size: 20.0,
                                    color: TEXT_COLOR,
                                    ..default()
                                },
                            ));
                        });
                }
            });
    }
}

fn text_bio_system(
    mut commands: Commands,
    query_entity: Query<Entity, With<UiBioPanel>>,
    bio: Res<Bio>,
) {
    if bio.is_changed() {
        let entity = query_entity.single();

        commands.entity(entity).despawn_descendants();

        commands
            .entity(entity)
            .insert((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::clip_y(),
                        ..default()
                    },
                    ..default()
                },
                ScrollingList::default(),
            ))
            .with_children(|parent| {
                let mut last_age = -1.0;

                for (age, bio, times) in &**bio {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    if last_age == *age {
                                        "".to_owned()
                                    } else {
                                        format!("{}", age)
                                    },
                                    TextStyle {
                                        font_size: 16.0,
                                        color: TEXT_COLOR,
                                        ..default()
                                    },
                                )
                                .with_style(Style {
                                    width: Val::Percent(15.0),
                                    ..default()
                                }),
                            );

                            parent.spawn(
                                TextBundle::from_section(
                                    if *times > 1 {
                                        format!("{} * {}", bio, times)
                                    } else {
                                        bio.to_owned()
                                    },
                                    TextStyle {
                                        font_size: 16.0,
                                        color: TEXT_COLOR,
                                        ..default()
                                    },
                                )
                                .with_style(Style {
                                    width: Val::Percent(85.0),
                                    ..default()
                                }),
                            );
                        });

                    last_age = *age;
                }
            });
    }
}

fn text_stat_int_system(
    (query_stat, mut query_text): (
        Query<&PlayerStat, Changed<PlayerStat>>,
        Query<&mut Text, With<UiPlayerStatIntuitionLabel>>,
    ),
) {
    if let Ok(stat) = query_stat.get_single() {
        for mut text in &mut query_text {
            text.sections[1].value = format!("{:.0}", stat.intuition);
        }
    }
}

fn text_stat_kno_system(
    (query_stat, mut query_text): (
        Query<&PlayerStat, Changed<PlayerStat>>,
        Query<&mut Text, With<UiPlayerStatKnowledgeLabel>>,
    ),
) {
    if let Ok(stat) = query_stat.get_single() {
        for mut text in &mut query_text {
            text.sections[1].value = format!("{:.0}", stat.knowledge);
        }
    }
}

fn text_stat_phy_system(
    (query_stat, mut query_text): (
        Query<&PlayerStat, Changed<PlayerStat>>,
        Query<&mut Text, With<UiPlayerStatPhysicalLabel>>,
    ),
) {
    if let Ok(stat) = query_stat.get_single() {
        for mut text in &mut query_text {
            text.sections[1].value = format!("{:.0}", stat.physical);
        }
    }
}

fn text_stat_soc_system(
    (query_stat, mut query_text): (
        Query<&PlayerStat, Changed<PlayerStat>>,
        Query<&mut Text, With<UiPlayerStatSocialLabel>>,
    ),
) {
    if let Ok(stat) = query_stat.get_single() {
        for mut text in &mut query_text {
            text.sections[1].value = format!("{:.0}", stat.social);
        }
    }
}

fn trifle_spawn_system(
    mut commands: Commands,
    query_player: Query<(&Age, &Attributable, &EmotionalRating, &PlayerStat), With<Player>>,
    (game_assets, asset_handles, time): (Res<Assets<GameAsset>>, Res<GameDataAssets>, Res<Time>),
    mut timer: ResMut<TrifleSpawnTimer>,
) {
    let (age, attrs, er, stats) = query_player.single();

    if timer.tick(time.delta()).finished() {
        let game_asset = game_assets.get(&asset_handles.core).unwrap();

        let category_and_lot = (*game_asset).query_category_and_lot(&QueryInfo {
            age: age.0,
            attrs: &attrs,
            er: &er,
            stats: &stats,
        });

        if let Some((category, lot)) = category_and_lot {
            let normal = Normal::new(0.5, 0.1).unwrap();
            let p = normal.sample(&mut rand::thread_rng());
            let p = (p as f32).clamp(0.1, 0.9);

            let lot_desc = if lot.hidden {
                format!("????")
            } else if lot.hide_desc {
                format!("{} - ????", category.name)
            } else {
                format!("{} - {}", category.name, lot.desc)
            };

            let width = GAME_AREA_WIDTH * p * 0.5;
            let size = Vec2::new(width, TRIFLE_HEIGHT);

            let mut x = rand::thread_rng().gen_range(0.0..GAME_AREA_WIDTH) - GAME_AREA_WIDTH * 0.5;

            let left_edge = x - width * 0.5;
            if left_edge < -GAME_AREA_WIDTH * 0.5 {
                x = (width - GAME_AREA_WIDTH) * 0.5;
            }

            let right_edge = x + width * 0.5;
            if right_edge > GAME_AREA_WIDTH * 0.5 {
                x = (GAME_AREA_WIDTH - width) * 0.5;
            }

            let translation = Vec3::new(x, (GAME_AREA_HEIGHT - TRIFLE_HEIGHT) * 0.5, 1.0);

            commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(size),
                            ..default()
                        },
                        transform: Transform::from_translation(translation),
                        ..default()
                    },
                    Trifle(lot.clone()),
                    CanHappen(false),
                    Speed(rand::thread_rng().gen_range(150.0..200.0)),
                    OnGameScreen,
                ))
                .with_children(|parent| {
                    parent.spawn(Text2dBundle {
                        text: Text::from_section(
                            lot_desc,
                            TextStyle {
                                font_size: TRIFLE_LABEL_FONT_SIZE,
                                color: Color::DARK_GRAY,
                                ..default()
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                        transform: Transform::from_translation(Vec3::new(
                            0.0,
                            size.y * 0.5 + 4.0 + TRIFLE_LABEL_FONT_SIZE * 0.5,
                            100.0,
                        )),
                        ..default()
                    });
                });
        }
    }
}

fn trifle_handle_system(
    mut commands: Commands,
    (mut query_player, query_trifle): (
        Query<
            (
                &Age,
                &mut Attributable,
                &mut EmotionalRating,
                &mut PlayerStat,
                &Transform,
            ),
            With<Player>,
        >,
        Query<(Entity, &CanHappen, &Sprite, &Transform, &Trifle), (With<Trifle>, Without<Player>)>,
    ),
    (audio_assets, game_assets, asset_handles, volume): (
        Res<AudioAssets>,
        Res<Assets<GameAsset>>,
        Res<GameDataAssets>,
        Res<Volume>,
    ),
    (mut res_attrs, mut bio): (ResMut<Attributes>, ResMut<Bio>),
) {
    let (age, mut attrs, mut er, mut stats, player_transform) = query_player.single_mut();
    let player_left = player_transform.translation.x - PLAYER_SIZE.x * 0.5;
    let player_right = player_transform.translation.x + PLAYER_SIZE.x * 0.5;

    for (entity, can_happen, sprite, transform, trifle) in query_trifle.iter() {
        if !**can_happen {
            continue;
        }

        let size = sprite.custom_size.unwrap();

        let left = transform.translation.x - size.x * 0.5;
        let right = transform.translation.x + size.x * 0.5;

        // Is intersected
        if player_left <= right && player_right >= left {
            let query = QueryInfo {
                age: age.0,
                attrs: &attrs,
                er: &er,
                stats: &stats,
            };

            let lot = &**trifle;
            let resp = lot.apply(&query);

            if let Some(er) = &resp.er {
                if er.lol >= 0.0 {
                    commands.spawn(AudioBundle {
                        source: audio_assets.lol.clone(),
                        settings: PlaybackSettings {
                            volume: audio::Volume::new_relative(volume.to_volume()),
                            ..default()
                        },
                    });
                } else {
                    commands.spawn(AudioBundle {
                        source: audio_assets.tot.clone(),
                        settings: PlaybackSettings {
                            volume: audio::Volume::new_relative(volume.to_volume()),
                            ..default()
                        },
                    });
                }
            } else {
                commands.spawn(AudioBundle {
                    source: audio_assets.lot.clone(),
                    settings: PlaybackSettings {
                        volume: audio::Volume::new_relative(volume.to_volume()),
                        ..default()
                    },
                });
            }

            apply_resp(
                resp,
                &game_assets,
                &asset_handles,
                &lot.desc,
                **age,
                &mut attrs,
                &mut er,
                &mut stats,
                &mut res_attrs,
                &mut bio,
                false,
            );

            commands.entity(entity).despawn_recursive();
        }
    }
}

fn trifle_miss_system(
    mut commands: Commands,
    (mut query_player, query_trifle): (
        Query<
            (
                &Age,
                &mut Attributable,
                &mut EmotionalRating,
                &mut PlayerStat,
            ),
            With<Player>,
        >,
        Query<(Entity, &CanHappen, &Transform, &Trifle)>,
    ),
    (audio_assets, game_assets, asset_handles, volume): (
        Res<AudioAssets>,
        Res<Assets<GameAsset>>,
        Res<GameDataAssets>,
        Res<Volume>,
    ),
    (mut res_attrs, mut bio): (ResMut<Attributes>, ResMut<Bio>),
) {
    let (age, mut attrs, mut er, mut stats) = query_player.single_mut();

    for (entity, can_happen, transform, trifle) in query_trifle.iter() {
        if !**can_happen {
            continue;
        }

        if transform.translation.y <= -(GAME_AREA_HEIGHT + TRIFLE_HEIGHT) * 0.5 {
            let query = QueryInfo {
                age: age.0,
                attrs: &attrs,
                er: &er,
                stats: &stats,
            };

            let lot = &**trifle;
            let resp = lot.apply_miss(&query);

            commands.spawn(AudioBundle {
                source: audio_assets.miss.clone(),
                settings: PlaybackSettings {
                    volume: audio::Volume::new_relative(volume.to_volume()),
                    ..default()
                },
            });

            apply_resp(
                resp,
                &game_assets,
                &asset_handles,
                &lot.desc,
                **age,
                &mut attrs,
                &mut er,
                &mut stats,
                &mut res_attrs,
                &mut bio,
                true,
            );

            commands.entity(entity).despawn_recursive();
        }
    }
}

fn trifle_update_system(
    mut query_trifle: Query<(&mut CanHappen, &Speed, &mut Transform), With<Trifle>>,
    time: Res<Time>,
) {
    for (mut can_happen, speed, mut transform) in query_trifle.iter_mut() {
        let new_position = transform.translation.y - speed.0 * time.delta_seconds();

        transform.translation.y = new_position.clamp(
            -(GAME_AREA_HEIGHT + TRIFLE_HEIGHT) * 0.5,
            (GAME_AREA_HEIGHT - TRIFLE_HEIGHT) * 0.5,
        );

        if transform.translation.y <= -GAME_AREA_HEIGHT * 0.5 + PLAYER_SIZE.y {
            **can_happen = true;
        }
    }
}

fn apply_resp(
    resp: RespInfo,
    game_assets: &Assets<GameAsset>,
    asset_handles: &GameDataAssets,
    lot_desc: &String,
    age: f32,
    attrs: &mut Attributable,
    er: &mut EmotionalRating,
    stats: &mut PlayerStat,
    res_attrs: &mut Attributes,
    bio: &mut Bio,
    missed: bool,
) {
    if let Some(ids) = resp.attrs {
        let game_asset = game_assets.get(&asset_handles.core).unwrap();

        let mut new_attrs = ids
            .iter()
            .map_while(|id| game_asset.get_attr(*id))
            .filter(|attr| !attr.hidden)
            .collect::<Vec<&Attribute>>();

        new_attrs.sort_by(|a, b| a.id.cmp(&b.id));

        let new_attrs = new_attrs
            .iter()
            .map(|attr| attr.name.clone())
            .collect::<Vec<String>>();

        res_attrs.0 = new_attrs;

        attrs.0 = ids;
    }

    if let Some(new_er) = resp.er {
        er.tot = new_er.tot;

        er.lol = new_er.lol.min(ER_CAPACITY - er.tot);
    }

    if let Some(new_stats) = resp.stats {
        stats.0 = new_stats;
    }

    let desc = if missed {
        format!("(Did Not) {}", lot_desc)
    } else {
        lot_desc.clone()
    };

    let event_repeated = bio
        .last_mut()
        .and_then(|last| {
            if last.0 == age && last.1 == *desc {
                last.2 += 1;

                return Some(true);
            }

            Some(false)
        })
        .unwrap_or(false);

    if !event_repeated {
        bio.push((age, desc, 1));
    }
}
