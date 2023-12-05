use bevy::prelude::*;

use crate::{despawn_screen, plugins::game::bundle::TrifleBundle, GameState};

use super::{
    bundle::StatBundle,
    component::{
        Age, OnGameScreen, Player, PlayerStat, Speed, Trifle, UiAgeLabel, UiGameArea,
        UiPlayerStatIntuitionLabel, UiPlayerStatKnowledgeLabel, UiPlayerStatPhysicalLabel,
        UiPlayerStatSocialLabel,
    },
    consntant::{
        GAME_AREA_BORDER_COLOR, GAME_AREA_BORDER_WIDTH, GAME_AREA_WIDTH, PANEL_BACKGROUND_COLOR,
        PANEL_WIDTH, PLAYER_SIZE,
    },
    resource::TrifleSpawnTimer,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter
            .add_systems(OnEnter(GameState::Game), setup)
            // Update
            .add_systems(
                Update,
                (keyboard_system, mouse_system).run_if(in_state(GameState::Game)),
            )
            .add_systems(
                Update,
                (aging_system, trifle_system).run_if(in_state(GameState::Game)),
            )
            .add_systems(
                Update,
                (
                    text_age_system,
                    text_stat_int_system,
                    text_stat_kno_system,
                    text_stat_phy_system,
                    text_stat_soc_system,
                )
                    .run_if(in_state(GameState::Game)),
            )
            // OnExit
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

fn setup(mut commands: Commands) {
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
                        width: Val::Px(PANEL_WIDTH),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: PANEL_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Physical Stat
                    parent.spawn((StatBundle::new("PHY"), UiPlayerStatPhysicalLabel));

                    // // Intuition Stat
                    parent.spawn((StatBundle::new("INT"), UiPlayerStatIntuitionLabel));

                    // // Knowledge Stat
                    parent.spawn((StatBundle::new("KNO"), UiPlayerStatKnowledgeLabel));

                    // // Social Stat
                    parent.spawn((StatBundle::new("SOC"), UiPlayerStatSocialLabel));
                });

            // Center
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(GAME_AREA_WIDTH),
                        height: Val::Percent(100.0),
                        border: UiRect::all(Val::Px(GAME_AREA_BORDER_WIDTH)),
                        ..default()
                    },
                    border_color: GAME_AREA_BORDER_COLOR.into(),
                    ..default()
                },
                UiGameArea,
            ));

            // Right Panel
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(PANEL_WIDTH),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: PANEL_BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Age
                    parent.spawn((
                        TextBundle::from_sections([
                            TextSection::new(
                                "Age: ",
                                TextStyle {
                                    font_size: 40.0,
                                    ..default()
                                },
                            ),
                            TextSection::from_style(TextStyle {
                                font_size: 40.0,
                                color: Color::BLUE,
                                ..default()
                            }),
                        ]),
                        UiAgeLabel,
                    ));
                });
        });

    // Player
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                ..default()
            },
            transform: Transform {
                scale: PLAYER_SIZE,
                ..default()
            },
            ..default()
        },
        Player,
        PlayerStat::default(),
        Age(0),
        Speed(100.0),
        OnGameScreen,
    ));

    commands.insert_resource(TrifleSpawnTimer(Timer::from_seconds(
        1.0,
        TimerMode::Repeating,
    )));
}

fn aging_system() {}

fn keyboard_system(
    mut query_player: Query<(&mut Transform, &Speed), With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, speed) = query_player.single_mut();
    let speed = speed.0;

    let mut direction = 0.0;

    if input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let new_position = transform.translation.x + direction * speed * time.delta_seconds();

    let left_bound = -GAME_AREA_WIDTH * 0.5 + PLAYER_SIZE.x * 0.5;
    let right_bound = GAME_AREA_WIDTH * 0.5 - PLAYER_SIZE.x * 0.5;

    transform.translation.x = new_position.clamp(left_bound, right_bound);
}

fn mouse_system(input: Res<Input<MouseButton>>) {
    if input.just_pressed(MouseButton::Left) {
        // Left button was pressed
    }
}

fn text_age_system(
    mut query_text: Query<&mut Text, With<UiAgeLabel>>,
    query_age: Query<&Age, With<Player>>,
) {
    let age = query_age.single();

    for mut text in &mut query_text {
        text.sections[1].value = format!("{:.0}", age.0);
    }
}

fn text_stat_int_system(
    mut query_text: Query<&mut Text, With<UiPlayerStatIntuitionLabel>>,
    query_stat: Query<&PlayerStat>,
) {
    let stat = query_stat.single();

    for mut text in &mut query_text {
        text.sections[1].value = format!("{:.0}", stat.intuition);
    }
}

fn text_stat_kno_system(
    mut query_text: Query<&mut Text, With<UiPlayerStatKnowledgeLabel>>,
    query_stat: Query<&PlayerStat>,
) {
    let stat = query_stat.single();

    for mut text in &mut query_text {
        text.sections[1].value = format!("{:.0}", stat.knowledge);
    }
}

fn text_stat_phy_system(
    mut query_text: Query<&mut Text, With<UiPlayerStatPhysicalLabel>>,
    query_stat: Query<&PlayerStat>,
) {
    let stat = query_stat.single();

    for mut text in &mut query_text {
        text.sections[1].value = format!("{:.0}", stat.physical);
    }
}

fn text_stat_soc_system(
    mut query_text: Query<&mut Text, With<UiPlayerStatSocialLabel>>,
    query_stat: Query<&PlayerStat>,
) {
    let stat = query_stat.single();

    for mut text in &mut query_text {
        text.sections[1].value = format!("{:.0}", stat.social);
    }
}

fn trifle_system(
    mut commands: Commands,
    query_trifle: Query<&Trifle>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<TrifleSpawnTimer>,
) {
    if query_trifle.is_empty() {
        commands.spawn(TrifleBundle::new(SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            ..default()
        }));
    }

    if timer.tick(time.delta()).finished() {
        commands.spawn(TrifleBundle::new(SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            ..default()
        }));
    }
}
