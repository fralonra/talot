use bevy::{prelude::*, utils::HashSet};
use rand::Rng;
use rand_distr::{Distribution, Normal};
use talot_core::QueryInfo;

use crate::{
    asset::GameAsset, common::despawn_screen, resource::GameAssetHandle, state::GameState,
};

use super::{
    bundle::StatBundle,
    component::{
        Age, Attributable, OnGameScreen, Player, PlayerStat, Speed, Trifle, UiAgeLabel, UiGameArea,
        UiPlayerStatIntuitionLabel, UiPlayerStatKnowledgeLabel, UiPlayerStatPhysicalLabel,
        UiPlayerStatSocialLabel,
    },
    constant::{
        GAME_AREA_HEIGHT, GAME_AREA_WIDTH, PANEL_BACKGROUND_COLOR, PANEL_WIDTH, PLAYER_SIZE,
        TRIFLE_HEIGHT, TRIFLE_LABEL_FONT_SIZE,
    },
    resource::{AgingTimer, TrifleSpawnTimer},
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
            .add_systems(Update, (aging_system).run_if(in_state(GameState::Game)))
            .add_systems(
                Update,
                (
                    trifle_spawn_system,
                    trifle_update_system,
                    trifle_handle_system,
                )
                    .run_if(in_state(GameState::Game)),
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
            // parent.spawn(NodeBundle {
            //     style: Style {
            //         width: Val::Px(GAME_AREA_WIDTH + GAME_AREA_BORDER_WIDTH),
            //         height: Val::Px(GAME_AREA_HEIGHT + GAME_AREA_BORDER_WIDTH),
            //         ..default()
            //     },
            //     background_color: GAME_AREA_BORDER_COLOR.into(),
            //     ..default()
            // });

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

    // Game Area
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(Vec2::new(GAME_AREA_WIDTH, GAME_AREA_HEIGHT)),
                ..default()
            },
            ..default()
        },
        UiGameArea,
    ));

    // Player
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                0.0,
                (PLAYER_SIZE.y - GAME_AREA_HEIGHT) * 0.5,
                1.0,
            )),
            ..default()
        },
        Player,
        PlayerStat::default(),
        Attributable(HashSet::new()),
        Age(0.0),
        Speed(100.0),
        OnGameScreen,
    ));

    commands.insert_resource(AgingTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));

    commands.insert_resource(TrifleSpawnTimer(Timer::from_seconds(
        2.0,
        TimerMode::Repeating,
    )));
}

fn aging_system(
    mut query_age: Query<&mut Age>,
    time: Res<Time>,
    mut timer: ResMut<TrifleSpawnTimer>,
) {
    let mut age = query_age.single_mut();

    if timer.tick(time.delta()).finished() {
        **age += 1.0;
    }
}

fn keyboard_system(
    mut query_player: Query<(&Speed, &mut Transform), With<Player>>,
    (input, time): (Res<Input<KeyCode>>, Res<Time>),
) {
    let (speed, mut transform) = query_player.single_mut();
    let speed = speed.0;

    let mut direction = 0.0;

    if input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let new_position = transform.translation.x + direction * speed * time.delta_seconds();

    let left_bound = (PLAYER_SIZE.x - GAME_AREA_WIDTH) * 0.5;
    let right_bound = (GAME_AREA_WIDTH - PLAYER_SIZE.x) * 0.5;

    transform.translation.x = new_position.clamp(left_bound, right_bound);
}

fn mouse_system(input: Res<Input<MouseButton>>) {
    if input.just_pressed(MouseButton::Left) {
        // Left button was pressed
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
    query_player: Query<(&Age, &Attributable, &PlayerStat), With<Player>>,
    (asset_server, game_asset_handle, time): (Res<AssetServer>, Res<GameAssetHandle>, Res<Time>),
    (game_assets, mut timer): (ResMut<Assets<GameAsset>>, ResMut<TrifleSpawnTimer>),
) {
    let (age, attrs, stats) = query_player.single();

    if timer.tick(time.delta()).finished() {
        let normal = Normal::new(0.5, 0.1).unwrap();
        let p = normal.sample(&mut rand::thread_rng());
        let p = (p as f32).clamp(0.1, 0.9);

        let game_asset = game_assets.get(&game_asset_handle.0).unwrap();
        let lot = (*game_asset).get_lot(&QueryInfo {
            age: **age,
            attrs: &attrs.to_vec(),
            stats: &stats,
        });

        if let Some(lot) = lot {
            let lot_desc = lot.desc.clone();

            let width = GAME_AREA_WIDTH * p;
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
                    Trifle::new(lot),
                    Speed(rand::thread_rng().gen_range(50.0..100.0)),
                ))
                .with_children(|parent| {
                    parent.spawn(Text2dBundle {
                        text: Text::from_section(
                            lot_desc,
                            TextStyle {
                                font_size: TRIFLE_LABEL_FONT_SIZE,
                                color: Color::GOLD,
                                ..default()
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                        transform: Transform::from_translation(Vec3::new(
                            0.0,
                            size.y * 0.5 + 4.0 + TRIFLE_LABEL_FONT_SIZE * 0.5,
                            0.0,
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
        Query<(&mut Sprite, &Transform), With<Player>>,
        Query<(Entity, &Sprite, &Transform, &Trifle), Without<Player>>,
    ),
    time: Res<Time>,
) {
    let (mut player_sprite, player_transform) = query_player.single_mut();
    let player_left = player_transform.translation.x - PLAYER_SIZE.x * 0.5;
    let player_right = player_transform.translation.x + PLAYER_SIZE.x * 0.5;

    let mut is_intersected = false;

    for (entity, sprite, transform, trifle) in query_trifle.iter() {
        if !trifle.can_happend {
            continue;
        }

        let size = sprite.custom_size.unwrap();

        let left = transform.translation.x - size.x * 0.5;
        let right = transform.translation.x + size.x * 0.5;

        if player_left <= right && player_right >= left {
            is_intersected = true;

            player_sprite.color = Color::RED;

            commands.entity(entity).despawn_recursive();
        }
    }

    if !is_intersected {
        player_sprite.color = Color::rgb(0.25, 0.25, 0.75);
    }
}

fn trifle_update_system(
    mut query_trifle: Query<(&mut Sprite, &Speed, &mut Transform, &mut Trifle)>,
    time: Res<Time>,
) {
    for (mut sprite, speed, mut transform, mut trifle) in query_trifle.iter_mut() {
        let new_position = transform.translation.y - speed.0 * time.delta_seconds();

        transform.translation.y = new_position.clamp(
            (TRIFLE_HEIGHT - GAME_AREA_HEIGHT) * 0.5,
            (GAME_AREA_HEIGHT - TRIFLE_HEIGHT) * 0.5,
        );

        if transform.translation.y <= -GAME_AREA_HEIGHT * 0.5 + PLAYER_SIZE.y {
            trifle.can_happend = true;

            sprite.color = Color::GREEN;
        }
    }
}
