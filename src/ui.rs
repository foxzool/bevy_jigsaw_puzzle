use crate::gameplay::{JigsawPuzzleGenerator, MoveTogether, Selected};
use crate::Piece;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        // .add_systems(Startup, setup_ui)
        .add_event::<AdjustScale>()
        .add_event::<ToggleBackgroundHint>()
        .add_event::<TogglePuzzleHint>()
        .add_systems(
            Update,
            (
                adjust_camera_on_added_sprite,
                adjust_camera_scale,
                handle_keyboard_input,
                handle_mouse_wheel_input,
                handle_toggle_background_hint,
                handle_toggle_puzzle_hint,
            ),
        );
}

#[derive(Component)]
pub struct MenuIcon;
#[derive(Component)]
pub struct ZoomInButton;
#[derive(Component)]
pub struct ZoomOutButton;
#[derive(Component)]
pub struct HintImageButton;
#[derive(Component)]
pub struct FullscreenButton;
#[derive(Component)]
pub struct PauseButton;
#[derive(Component)]
pub struct IdeaButton;
#[derive(Component)]
pub struct PuzzleHintButton;
#[derive(Component)]
pub struct PuzzleHintChildButton;
#[derive(Component)]
pub struct BackgroundHintButton;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[allow(dead_code)]
fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // let background_color = MAROON.into();
    let root_node = commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        })
        .id();

    let left_column = commands
        .spawn(Node {
            width: Val::Vw(15.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Start,
            margin: UiRect::axes(Val::Px(15.), Val::Px(5.)),
            ..default()
        })
        .with_children(|builder| {
            // top left
            builder
                .spawn((
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Px(50.),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    // BackgroundColor(BLUE.into()),
                ))
                .with_children(|builder| {
                    builder.spawn((
                        UiImage::new(asset_server.load("icons/menu.png")),
                        Node {
                            height: Val::Px(40.),
                            ..default()
                        },
                        MenuIcon,
                    ));
                    builder
                        .spawn(Node {
                            height: Val::Px(30.0),
                            justify_content: JustifyContent::End,
                            ..default()
                        })
                        .with_children(|builder| {
                            builder.spawn((
                                UiImage::new(asset_server.load("icons/zoom_out.png")),
                                Node {
                                    height: Val::Px(30.),
                                    margin: UiRect {
                                        left: Val::Px(5.),
                                        right: Val::Px(5.),
                                        ..default()
                                    },
                                    ..default()
                                },
                                ZoomOutButton,
                            ));
                            builder.spawn((
                                UiImage::new(asset_server.load("icons/zoom_in.png")),
                                Node {
                                    height: Val::Px(30.),
                                    margin: UiRect {
                                        left: Val::Px(5.),
                                        right: Val::Px(5.),
                                        ..default()
                                    },
                                    ..default()
                                },
                                ZoomInButton,
                            ));
                        });
                });

            // bottom left
            builder.spawn(Node::default()).with_children(|p| {
                // idea
                p.spawn((
                    UiImage::new(asset_server.load("icons/lamp.png")),
                    Node {
                        height: Val::Px(40.),
                        margin: UiRect::axes(Val::Px(0.), Val::Px(5.)),
                        ..default()
                    },
                    IdeaButton,
                ));

                // puzzle control
                p.spawn((
                    Node {
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    },
                    PuzzleHintButton,
                ))
                .with_children(|p| {
                    p.spawn((
                        UiImage {
                            image: asset_server.load("icons/puzzle_s.png"),
                            flip_x: true,
                            ..default()
                        },
                        Node {
                            height: Val::Px(40.),
                            margin: UiRect::axes(Val::Px(2.), Val::Px(5.)),
                            ..default()
                        },
                    ));

                    p.spawn((
                        UiImage::new(asset_server.load("icons/puzzle_e.png")),
                        Node {
                            height: Val::Px(30.),
                            margin: UiRect {
                                top: Val::Px(10.),
                                bottom: Val::Px(10.),

                                ..default()
                            },
                            ..default()
                        },
                        PuzzleHintChildButton,
                    ));

                    p.spawn((
                        UiImage::new(asset_server.load("icons/puzzle_s.png")),
                        Node {
                            height: Val::Px(40.),
                            margin: UiRect::axes(Val::Px(2.), Val::Px(5.)),
                            ..default()
                        },
                    ));
                });

                // background hint
                p.spawn((
                    UiImage::new(asset_server.load("icons/ghost.png")),
                    Node {
                        height: Val::Px(40.),
                        margin: UiRect::axes(Val::Px(0.), Val::Px(5.)),
                        ..default()
                    },
                    BackgroundHintButton,
                ));
            });
        })
        .id();

    let right_column = commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::End,
            margin: UiRect::axes(Val::Px(15.), Val::Px(5.)),
            ..default()
        })
        .with_children(|builder| {
            // top right
            builder.spawn((
                UiImage::new(asset_server.load("icons/photo.png")),
                Node {
                    height: Val::Px(40.),
                    margin: UiRect::all(Val::Px(5.)),
                    ..default()
                },
                HintImageButton,
            ));

            // bottom right
            builder.spawn(Node::default()).with_children(|p| {
                p.spawn((
                    UiImage::new(asset_server.load("icons/pause.png")),
                    Node {
                        height: Val::Px(40.),
                        margin: UiRect {
                            right: Val::Px(10.),
                            ..default()
                        },
                        ..default()
                    },
                    PauseButton,
                ));
                p.spawn((
                    UiImage::new(asset_server.load("icons/fullscreen.png")),
                    Node {
                        height: Val::Px(40.),
                        ..default()
                    },
                    FullscreenButton,
                ));
            });
        })
        .id();
    commands
        .entity(root_node)
        .add_children(&[left_column, right_column]);
}

#[derive(Component)]
pub struct BoardBackgroundImage;

/// Adjust the camera to fit the image
fn adjust_camera_on_added_sprite(
    _sprite: Single<Entity, Added<BoardBackgroundImage>>,
    mut camera_2d: Single<&mut OrthographicProjection, With<Camera2d>>,
    window: Single<&Window>,
    generator: Res<JigsawPuzzleGenerator>,
) {
    let window_width = window.resolution.width();
    let image_width = generator.origin_image().width() as f32;
    let scale = image_width / window_width;
    let target_scale = scale / 0.6;
    camera_2d.scale = target_scale;
}

#[derive(Event)]
pub struct AdjustScale(pub f32);

const MAX_SCALE: f32 = 3.0;
const MIN_SCALE: f32 = 0.5;

/// Adjust the camera scale on event
fn adjust_camera_scale(
    mut event: EventReader<AdjustScale>,
    mut camera_2d: Single<&mut OrthographicProjection, With<Camera2d>>,
) {
    for AdjustScale(scale) in event.read() {
        let new_scale = camera_2d.scale + scale;
        debug!("new scale: {}", new_scale);
        if (MIN_SCALE..=MAX_SCALE).contains(&new_scale) {
            camera_2d.scale = new_scale;
        }
    }
}

fn handle_keyboard_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if keyboard_input.just_pressed(KeyCode::PageUp) {
        commands.send_event(AdjustScale(0.1));
    } else if keyboard_input.just_pressed(KeyCode::PageDown) {
        commands.send_event(AdjustScale(-0.1));
    } else if keyboard_input.just_pressed(KeyCode::Space) {
        commands.send_event(ToggleBackgroundHint);
    } else if keyboard_input.just_pressed(KeyCode::KeyH) {
        commands.send_event(TogglePuzzleHint);
    }
}

fn handle_mouse_wheel_input(
    mut mouse_wheel_input: EventReader<MouseWheel>,
    mut commands: Commands,
) {
    for event in mouse_wheel_input.read() {
        commands.send_event(AdjustScale(event.y * 0.1));
    }
}

#[derive(Event)]
pub struct ToggleBackgroundHint;

fn handle_toggle_background_hint(
    mut event: EventReader<ToggleBackgroundHint>,
    mut query: Query<&mut Visibility, With<BoardBackgroundImage>>,
) {
    for _ in event.read() {
        for mut visible in query.iter_mut() {
            visible.toggle_visible_hidden();
        }
    }
}

#[derive(Event)]
pub struct TogglePuzzleHint;

fn handle_toggle_puzzle_hint(
    mut event: EventReader<TogglePuzzleHint>,
    selected_query: Query<Entity, With<Selected>>,
    piece_query: Query<(Entity, &Piece, &MoveTogether), Without<Selected>>,
    mut commands: Commands,
) {
    for _ in event.read() {
        for entity in selected_query.iter() {
            commands.entity(entity).remove::<Selected>();
        }
        let mut first_piece = None;
        let mut first_entity = None;
        let mut second_entity = None;
        for (entity, piece, move_together) in piece_query.iter() {
            if move_together.len() > 0 {
                continue;
            }
            first_piece = Some(piece);
            first_entity = Some(entity);
        }
        if let Some(first_piece) = first_piece {
            for (entity, piece, move_together) in piece_query.iter() {
                if move_together.len() > 0 {
                    continue;
                }
                if first_piece.beside(&piece) {
                    second_entity = Some(entity);
                }
            }
        }
        if let (Some(first_entity), Some(second_entity)) = (first_entity, second_entity) {
            commands.entity(first_entity).insert(Selected);
            commands.entity(second_entity).insert(Selected);
        }
    }
}
