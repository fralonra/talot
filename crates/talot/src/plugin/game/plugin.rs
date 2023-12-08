use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};
use rand::Rng;
use rand_distr::{Distribution, Normal};
use talot_core::QueryInfo;

use crate::{
    asset::{GameAsset, GameDataAssets},
    common::despawn_screen,
    state::GameState,
};

use super::{
    bundle::StatBundle,
    component::{
        Age, Attributable, EmotionalRating, OnGameScreen, Player, PlayerStat, ScrollingList, Speed,
        Trifle, UiAgeLabel, UiBioPanel, UiGameArea, UiPlayerStatIntuitionLabel,
        UiPlayerStatKnowledgeLabel, UiPlayerStatPhysicalLabel, UiPlayerStatSocialLabel,
    },
    constant::{
        GAME_AREA_HEIGHT, GAME_AREA_WIDTH, PANEL_BACKGROUND_COLOR, PANEL_WIDTH, PLAYER_SIZE,
        TRIFLE_HEIGHT, TRIFLE_LABEL_FONT_SIZE,
    },
    resource::{AgingTimer, Attributes, Bio, TrifleSpawnTimer},
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
                    text_attrs_system,
                    text_bio_system,
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
                        padding: UiRect::all(Val::Px(5.0)),
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
                                    font_size: 30.0,
                                    ..default()
                                },
                            ),
                            TextSection::from_style(TextStyle {
                                font_size: 30.0,
                                color: Color::BLUE,
                                ..default()
                            }),
                        ]),
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
                });

            // Right Panel
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(PANEL_WIDTH),
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
                            ..default()
                        },
                    ));

                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
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
        Age(0.0),
        Attributable::default(),
        EmotionalRating::default(),
        Speed(100.0),
        OnGameScreen,
    ));

    commands.insert_resource(AgingTimer(Timer::from_seconds(3.0, TimerMode::Repeating)));
    commands.insert_resource(TrifleSpawnTimer(Timer::from_seconds(
        0.5,
        TimerMode::Repeating,
    )));

    commands.insert_resource(Attributes::default());
    commands.insert_resource(Bio::default());
}

fn aging_system(mut query_age: Query<&mut Age>, time: Res<Time>, mut timer: ResMut<AgingTimer>) {
    let mut age = query_age.single_mut();

    if timer.tick(time.delta()).finished() {
        age.0 += 1.0;
    }
}

fn keyboard_system(
    mut query_player: Query<(&Speed, &mut Transform), With<Player>>,
    (input, time): (Res<Input<KeyCode>>, Res<Time>),
) {
    let (speed, mut transform) = query_player.single_mut();

    let mut direction = 0.0;

    if input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let new_position = transform.translation.x + direction * speed.0 * time.delta_seconds();

    let left_bound = (PLAYER_SIZE.x - GAME_AREA_WIDTH) * 0.5;
    let right_bound = (GAME_AREA_WIDTH - PLAYER_SIZE.x) * 0.5;

    transform.translation.x = new_position.clamp(left_bound, right_bound);
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

            let max_scroll = (items_height - container_height).max(0.);

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
        }
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

fn text_attrs_system(attrs: Res<Attributes>) {
    if attrs.is_changed() {}
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
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
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

        let lot = (*game_asset).get_lot(&QueryInfo {
            age: age.0,
            attrs: &attrs,
            er: &er,
            stats: &stats,
        });

        if let Some(lot) = lot {
            let normal = Normal::new(0.5, 0.1).unwrap();
            let p = normal.sample(&mut rand::thread_rng());
            let p = (p as f32).clamp(0.1, 0.9);

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
                    Speed(rand::thread_rng().gen_range(90.0..150.0)),
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
        Query<
            (
                &Age,
                &mut Attributable,
                &mut EmotionalRating,
                &mut PlayerStat,
                &mut Sprite,
                &Transform,
            ),
            With<Player>,
        >,
        Query<(Entity, &Sprite, &Transform, &Trifle), Without<Player>>,
    ),
    mut bio: ResMut<Bio>,
) {
    let (age, mut attrs, mut er, mut stats, mut player_sprite, player_transform) =
        query_player.single_mut();
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

            let query = QueryInfo {
                age: age.0,
                attrs: &attrs,
                er: &er,
                stats: &stats,
            };

            let lot = &trifle.lot;
            let resp = lot.apply(&query);

            if let Some(new_attrs) = resp.attrs {
                attrs.0 = new_attrs;
            }

            if let Some(new_er) = resp.er {
                er.0 = new_er;
            }

            if let Some(new_stats) = resp.stats {
                stats.0 = new_stats;
            }

            let event_repeated = bio
                .last_mut()
                .and_then(|last| {
                    if last.0 == age.0 && last.1 == lot.desc {
                        last.2 += 1;

                        return Some(true);
                    }

                    Some(false)
                })
                .unwrap_or(false);

            if !event_repeated {
                bio.push((age.0, lot.desc.clone(), 1));
            }

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
