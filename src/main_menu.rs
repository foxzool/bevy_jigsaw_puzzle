use crate::{
    despawn_screen, AnimeCamera, AppState, OriginImage, SelectGameMode, SelectPiece, UiCamera,
    ANIMATION_LAYERS,
};
use bevy::animation::{AnimationTarget, AnimationTargetId};
use bevy::color::palettes::basic::BLACK;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::ui::NodeType::Rect;
use bevy::window::WindowResized;

pub(crate) fn menu_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(AppState::MainMenu),
        (setup_camera, setup_menu).chain(),
    )
    .add_systems(
        Update,
        (
            windows_resize_event,
            menu_countdown,
            button_interaction,
            show_origin_image.run_if(resource_changed::<OriginImage>),
            update_piece_text.run_if(resource_changed::<SelectPiece>),
            update_game_mode_text.run_if(resource_changed::<SelectGameMode>),
        )
            .run_if(in_state(AppState::MainMenu)),
    )
    .add_systems(OnExit(AppState::MainMenu), despawn_screen::<OnMenuScreen>)
    .add_observer(show_title);
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Reflect)]
struct TextColorProperty;

#[derive(Component)]
struct OnMenuScreen;

#[derive(Resource, Deref, DerefMut)]
struct MenuTimer(Timer);

#[derive(Event)]
struct ShowTitleAnime;

impl AnimatableProperty for TextColorProperty {
    type Component = TextColor;

    type Property = Srgba;

    fn get_mut(component: &mut Self::Component) -> Option<&mut Self::Property> {
        match component.0 {
            Color::Srgba(ref mut color) => Some(color),
            _ => None,
        }
    }
}

fn setup_camera(mut commands: Commands) {
    let anime_camera = commands
        .spawn((
            Camera2d,
            Camera {
                order: 1,
                ..default()
            },
            ANIMATION_LAYERS,
        ))
        .id();

    commands.insert_resource(AnimeCamera(anime_camera));

    let ui_camera = commands.spawn(Camera2d).id();
    commands.insert_resource(UiCamera(ui_camera));
}

#[derive(Component)]
struct HiddenItem;

fn show_title(
    _trigger: Trigger<ShowTitleAnime>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut animations: ResMut<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    window: Single<&Window>,
    anime_camera: Res<AnimeCamera>,
    old_title: Query<Entity, With<AnimationTarget>>,
) {
    println!(
        "Setting up menu screen {}x{}",
        window.width(),
        window.height()
    );

    for entity in old_title.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let font = asset_server.load("fonts/MinecraftEvenings.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 55.0,
        ..default()
    };
    let text_justification = JustifyText::Center;

    let title = Name::new("title");

    let start_pos = (
        window.width() / -2.0 + window.width() * 0.2,
        window.height() / -2.0 + window.height() * 0.6,
    );

    // Creating the animation
    let mut animation = AnimationClip::default();
    // A curve can modify a single part of a transform: here, the translation.
    let title_animation_target_id = AnimationTargetId::from_name(&title);
    animation.add_curve_to_target(
        title_animation_target_id,
        UnevenSampleAutoCurve::new([0.0, 0.5, 1.0, 2.0, 3.0].into_iter().zip([
            Vec3::new(start_pos.0, start_pos.1, 0.0),
            Vec3::new(start_pos.0, start_pos.1, 0.0),
            Vec3::new(start_pos.0, start_pos.1 + 50.0, 0.0),
            Vec3::new(start_pos.0, start_pos.1 + 100.0, 0.0),
        ]))
        .map(TranslationCurve)
        .expect("should be able to build translation curve because we pass in valid samples"),
    );

    animation.add_curve_to_target(
        title_animation_target_id,
        AnimatableKeyframeCurve::new([0.0, 1.0, 2.0, 3.0].into_iter().zip([
            Srgba::new(0.0, 0.0, 0.0, 0.1),
            Srgba::new(0.0, 0.0, 0.0, 0.3),
            Srgba::new(0.0, 0.0, 0.0, 0.6),
            Srgba::new(0.0, 0.0, 0.0, 1.0),
        ]))
        .map(AnimatableCurve::<TextColorProperty, _>::from_curve)
        .expect("should be able to build translation curve because we pass in valid samples"),
    );

    // Create the animation graph
    let (graph, animation_index) = AnimationGraph::from_clip(animations.add(animation));

    // Create the animation player, and set it to repeat
    let mut player = AnimationPlayer::default();
    player.play(animation_index);

    let title_id = commands
        .spawn((
            Text2d::new("Jigsaw Puzzle"),
            text_font.clone(),
            TextLayout::new_with_justify(text_justification),
            TextColor(BLACK.into()),
            ANIMATION_LAYERS,
            TargetCamera(**anime_camera),
            Transform::from_xyz(start_pos.0, start_pos.1, 0.0),
            // Transform::from_xyz(0.0, 0.0, 0.0),
            title,
            AnimationGraphHandle(graphs.add(graph)),
            player,
            OnMenuScreen,
        ))
        .id();

    commands.entity(title_id).insert(AnimationTarget {
        id: title_animation_target_id,
        player: title_id,
    });
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_camera: Res<UiCamera>,
    select_piece: Res<SelectPiece>,
    select_mode: Res<SelectGameMode>,
) {
    let text_font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // let title_font = asset_server.load("fonts/MinecraftEvenings.ttf");
    let down_arrow = asset_server.load("icons/down-arrow.png");

    // Display the logo
    let root_node = commands
        .spawn((
            Node {
                // align_items: AlignItems::Center,
                // justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            UiImage::new(asset_server.load("images/puzzle.jpg")),
            TargetCamera(**ui_camera),
            OnMenuScreen,
        ))
        .id();

    let left_column = commands
        .spawn((
            Node {
                width: Val::Percent(40.),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            // BackgroundColor(Color::srgba(0.5, 0.0, 0.0, 0.5)),
            PickingBehavior::IGNORE,
            // Visibility::Hidden,
            HiddenItem,
        ))
        .with_children(|p| {
            // top container holder
            p.spawn(Node {
                height: Val::Percent(45.0),
                ..default()
            });
            // bottom container
            p.spawn((
                Node {
                    height: Val::Percent(55.0),
                    // width: Val::Px(300.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    // justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                // BackgroundColor(Color::srgba(0.5, 0.0, 0.0, 0.5)),
            ))
            .with_children(|p| {
                // selector container
                p.spawn((
                    Node {
                        // width: Val::Percent(100.0),
                        height: Val::Px(100.0),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    // BackgroundColor(Color::srgba(0.5, 0.0, 0.0, 0.5)),
                ))
                .with_children(|p| {
                    // piece selection
                    p.spawn(
                        (Node {
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::SpaceBetween,
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        }),
                    )
                    .with_children(|p| {
                        // up arrow
                        p.spawn((
                            UiImage {
                                image: down_arrow.clone(),
                                flip_y: true,
                                ..default()
                            },
                            Node {
                                width: Val::Px(30.0),
                                height: Val::Px(30.0),
                                ..default()
                            },
                        ))
                        .observe(
                            |_trigger: Trigger<Pointer<Click>>,
                             mut select_piece: ResMut<SelectPiece>| {
                                select_piece.previous();
                            },
                        );
                        p.spawn((
                            PieceNumText,
                            Text::new(select_piece.to_string()),
                            TextFont {
                                font: text_font.clone(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                            Node {
                                margin: UiRect::axes(Val::Px(10.0), Val::Px(0.0)),
                                ..default()
                            },
                        ));
                        // down arrow
                        p.spawn((
                            UiImage::new(down_arrow.clone()),
                            Node {
                                width: Val::Px(30.0),
                                height: Val::Px(30.0),
                                ..default()
                            },
                        ))
                        .observe(
                            |_trigger: Trigger<Pointer<Click>>,
                             mut select_piece: ResMut<SelectPiece>| {
                                select_piece.next();
                            },
                        );
                    });

                    // text
                    p.spawn((
                        Text::new("pieces"),
                        TextFont {
                            font: text_font.clone(),
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                        Node {
                            margin: UiRect::axes(Val::Px(0.0), Val::Px(31.0)),
                            ..default()
                        },
                    ));

                    // mode selection
                    p.spawn(
                        (Node {
                            height: Val::Percent(100.0),
                            justify_content: JustifyContent::SpaceBetween,
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        }),
                    )
                    .with_children(|p| {
                        // up arrow
                        p.spawn((
                            UiImage {
                                image: down_arrow.clone(),
                                flip_y: true,
                                ..default()
                            },
                            Node {
                                width: Val::Px(30.0),
                                height: Val::Px(30.0),
                                ..default()
                            },
                        ))
                        .observe(
                            |_trigger: Trigger<Pointer<Click>>,
                             mut select_mode: ResMut<SelectGameMode>| {
                                select_mode.previous();
                            },
                        );
                        p.spawn((
                            GameModeText,
                            Text::new(select_mode.to_string()),
                            TextFont {
                                font: text_font.clone(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::BLACK),
                            Node {
                                margin: UiRect::axes(Val::Px(10.0), Val::Px(0.0)),
                                ..default()
                            },
                        ));
                        // down arrow
                        p.spawn((
                            UiImage::new(down_arrow.clone()),
                            Node {
                                width: Val::Px(30.0),
                                height: Val::Px(30.0),
                                ..default()
                            },
                        ))
                        .observe(
                            |_trigger: Trigger<Pointer<Click>>,
                             mut select_mode: ResMut<SelectGameMode>| {
                                select_mode.next();
                            },
                        );
                    });
                });

                // start button
                p.spawn((
                    Button,
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        margin: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    // BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text::new("Start"),
                    TextFont {
                        font: text_font.clone(),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::BLACK),
                ))
                .observe(
                    |_trigger: Trigger<Pointer<Click>>,
                     mut game_state: ResMut<NextState<AppState>>| {
                        game_state.set(AppState::Gameplay)
                    },
                );
            });
        })
        .id();

    let right_column = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(60.0),
                height: Val::Percent(100.0),
                ..default()
            },
            PickingBehavior::IGNORE,
            // BackgroundColor(Color::srgba(0.5, 0.1, 0.0, 0.5)),
        ))
        .with_children(|p| {
            // image preview
            p.spawn((Node {
                width: Val::Percent(100.0),
                height: Val::Percent(70.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },))
                .with_children(|p| {
                    p.spawn((
                        HiddenItem,
                        Visibility::Hidden,
                        OriginImageContainer,
                        Node {
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        Outline {
                            width: Val::Px(5.0),
                            color: Color::BLACK,
                            offset: Val::Px(2.0),
                        },
                    ));
                });

            // game mode selection
            p.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(30.0),
                    display: Display::Flex,
                    justify_content: JustifyContent::SpaceBetween,
                    margin: UiRect {
                        left: Val::Px(4.),
                        ..default()
                    },
                    padding: UiRect::all(Val::Px(30.)),
                    ..default()
                },
                // BackgroundColor(Color::srgba(0.4, 0.5, 0.5, 0.5)),
            ))
            .with_children(|p| {
                // piece selection
                p.spawn((
                    Node {
                        width: Val::Percent(40.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.2, 0.7, 0.5, 0.5)),
                ));
            });
        })
        .id();

    commands
        .entity(root_node)
        .add_children(&[left_column, right_column]);

    commands.insert_resource(MenuTimer(Timer::from_seconds(2.9, TimerMode::Once)));
}

fn windows_resize_event(mut commands: Commands, mut resize_events: EventReader<WindowResized>) {
    for _ev in resize_events.read() {
        commands.trigger(ShowTitleAnime);
    }
}

#[derive(Component)]
struct OriginImageContainer;

fn menu_countdown(
    time: Res<Time>,
    mut timer: ResMut<MenuTimer>,
    mut items: Query<&mut Visibility, With<HiddenItem>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if timer.tick(time.delta()).just_finished() {
        for mut visible in items.iter_mut() {
            *visible = Visibility::Visible;
        }

        let image_handle = asset_server.load("../raw.jpg");

        commands.insert_resource(OriginImage(image_handle));
    }
}

fn button_interaction(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut TextColor>,
) {
    for (interaction, _color, _border_color, children) in &mut interaction_query {
        let mut text_color = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text_color.0 = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                // *color = Color::srgb(0.8, 0.8, 0.8).into();
                text_color.0 = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                text_color.0 = NORMAL_BUTTON.into();
                // *color = Color::srgba(0.0, 0.0, 0.0, 0.0).into();
                // border_color.0 = Color::BLACK;
            }
        }
    }
}

fn show_origin_image(
    container: Single<Entity, With<OriginImageContainer>>,
    mut commands: Commands,
    origin_image: Res<OriginImage>,
) {
    commands
        .entity(*container)
        .insert(UiImage::new(origin_image.0.clone()));
}

#[derive(Component)]
struct PieceNumText;

#[derive(Component)]
struct GameModeText;

fn update_game_mode_text(
    select_mode: Res<SelectGameMode>,

    mut mode_query: Query<&mut Text, With<GameModeText>>,
) {
    for mut text in mode_query.iter_mut() {
        text.0 = select_mode.to_string();
    }
}

fn update_piece_text(
    select_piece: Res<SelectPiece>,
    mut piece_query: Query<&mut Text, With<PieceNumText>>,
) {
    for mut text in piece_query.iter_mut() {
        text.0 = select_piece.to_string();
    }
}
