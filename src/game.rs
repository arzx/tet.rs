use crate::MenuCamera;
use crate::{BOARD_HEIGHT, BOARD_WIDTH, Board, Cell};
use bevy::prelude::*;
use bevy::window::Window;
use crate::tetrominoes::{ActivePiece, place_active_on_board, clear_active_from_board};


#[derive(Resource)]
pub struct FallTimer(pub Timer);

#[derive(Component)]
pub struct CellSprite {
    x: usize,
    y: usize,
}

//todo: 10x20 grid
pub fn setup_ingame(
    windows: Query<&Window>,
    mut commands: Commands,
    mut camera_query: Query<&mut Transform, With<MenuCamera>>,
) {
    if let Ok(mut transform) = camera_query.single_mut() {
        transform.translation = Vec3::new(0.0, 0.0, 0.0);
    }
    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(25.0),
            right: Val::Px(20.0),
            width: Val::Auto,
            height: Val::Auto,
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|col| {
                    col.spawn((Node {
                        width: Val::Px(220.0),
                        height: Val::Px(64.0),
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },))
                        .with_children(|col| {
                            col.spawn((
                                Text::new("Score:  "),
                                TextFont {
                                    font_size: 40.0,
                                    ..default()
                                },
                            ));
                        });
                    col.spawn((Node {
                        width: Val::Px(220.0),
                        height: Val::Px(64.0),
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },))
                        .with_children(|col| {
                            col.spawn((
                                Text::new("Next:  "),
                                TextFont {
                                    font_size: 40.0,
                                    ..default()
                                },
                            ));
                        });
                    col.spawn((Node {
                        width: Val::Px(220.0),
                        height: Val::Px(64.0),
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },))
                        .with_children(|col| {
                            col.spawn((
                                Text::new("Level:  "),
                                TextFont {
                                    font_size: 40.0,
                                    ..default()
                                },
                            ));
                        });
                });
        });
    let window = windows
        .single()
        .expect("primary window should exist before entering InGame");
    let board_width_px = window.width();
    let board_height_px = window.height();
    let cell_size =
        (board_width_px / BOARD_WIDTH as f32).min(board_height_px / BOARD_HEIGHT as f32);

    let board_pixel_width = cell_size * BOARD_WIDTH as f32;
    let board_pixel_height = cell_size * BOARD_HEIGHT as f32;
    let spacing_from_ui = 140.0;
    let offset_x = -board_pixel_width / 2.0 - spacing_from_ui + cell_size / 2.0;
    let offset_y = -board_pixel_height / 2.0 + cell_size / 2.0;

    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            commands.spawn((
                Sprite {
                    custom_size: Some(Vec2::splat(cell_size -1.0)),
                    color: Color::srgba(0.1, 0.1, 0.1, 0.5),
                    ..default()
                },
                Transform::from_xyz(
                    offset_x + x as f32 * cell_size,
                    offset_y + y as f32 * cell_size,
                    0.0,
                ),
                CellSprite { x, y },
            ));
        }
    }
}

pub fn sync_board(board: Res<Board>, mut cells: Query<(&CellSprite, &mut Sprite)>) {
    if board.is_changed() {
        for (cell_info, mut sprite) in &mut cells {
            match board.cells[cell_info.y][cell_info.x] {
                Cell::Empty => {
                    sprite.color = Color::srgba(0.0, 0.0, 0.0, 0.7); // faint grid
                }
                Cell::Filled(color) => {
                    sprite.color = color;
                }
            }
        }
    }
}

pub fn spawn_first_piece(
    mut commands: Commands,
    mut board: ResMut<Board>,
) {
    let active = ActivePiece::spawn_new();

    // use it first
    place_active_on_board(&active, &mut board, Color::srgb(0.8, 0.9, 1.0));

    // then move it into the world as a resource
    commands.insert_resource(active);
}

pub fn setup_fall_timer(mut commands: Commands) {
    commands.insert_resource(FallTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
}

pub fn fall_piece_system(
    time: Res<Time>,
    mut timer: ResMut<FallTimer>,
    mut board: ResMut<Board>,
    mut active: ResMut<ActivePiece>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    // 1) clear old position from the board
    clear_active_from_board(&active, &mut board);

    // 2) move piece down one cell (y - 1)
    active.y -= 1;

    // TODO later: stop when y < 0 or when hitting other blocks

    // 3) draw at new position
    place_active_on_board(&active, &mut board, Color::srgb(0.8, 0.9, 1.0));
}