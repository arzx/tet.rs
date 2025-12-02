use crate::MenuCamera;
use crate::{BOARD_HEIGHT, BOARD_WIDTH, Board, Cell};
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::window::Window;
use crate::tetrominoes::{ActivePiece, place_active_on_board, clear_active_from_board, shape_of};


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

    // draw it
    place_active_on_board(&active, &mut board);

    // store as resource
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

    // 1) clear current position FIRST so we don't collide with ourselves
    clear_active_from_board(&active, &mut board);

    // 2) check if we can move down one cell
    let shape = shape_of(active.kind.clone(), active.rotation);
    let mut can_move_down = true;

    for (dx, dy) in shape.cells {
        let new_x = active.x + dx;
        let new_y = active.y - 1 + dy;

        // bottom of board
        if new_y < 0 {
            can_move_down = false;
            break;
        }

        // hit existing block?
        if let Some(Cell::Filled(_)) = board.get(new_x, new_y) {
            can_move_down = false;
            break;
        }
    }

    if can_move_down {
        // 3a) move down and redraw
        active.y -= 1;
        place_active_on_board(&active, &mut board);
    } else {
        // 3b) lock in place where it was
        place_active_on_board(&active, &mut board);

        // 4) spawn a new random piece at the top
        *active = ActivePiece::spawn_new();
        place_active_on_board(&active, &mut board);

        // (optional later: check for game over if spawn collides)
    }
}

/// Move the active tetromino left/right in response to A/D key presses.
pub fn move_piece_horizontal_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut board: ResMut<Board>,
    mut active: ResMut<ActivePiece>,
) {
    // Determine horizontal movement: A = left (-1), D = right (+1)
    let mut dx = 0;
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        dx -= 1;
    }
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        dx += 1;
    }

    // No horizontal input this frame
    if dx == 0 {
        return;
    }

    // Temporarily clear current piece from the board so it doesn't collide with itself
    clear_active_from_board(&active, &mut board);

    // Check if we can move horizontally by `dx` without hitting walls or other blocks
    let shape = shape_of(active.kind.clone(), active.rotation);
    let mut can_move = true;

    for (cell_dx, cell_dy) in shape.cells {
        let new_x = active.x + cell_dx + dx;
        let new_y = active.y + cell_dy;

        // Check board bounds on X and Y
        if new_x < 0 || new_x >= BOARD_WIDTH as i32 || new_y < 0 {
            can_move = false;
            break;
        }

        // Check collision with existing filled cells
        if let Some(Cell::Filled(_)) = board.get(new_x, new_y) {
            can_move = false;
            break;
        }
    }

    // Apply movement if valid
    if can_move {
        active.x += dx;
    }

    // Redraw piece at its (potentially) new position
    place_active_on_board(&active, &mut board);
}

pub fn rotate_piece_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut board: ResMut<Board>,
    mut active: ResMut<ActivePiece>,
) {
    // Only act on a fresh W key press
    if !keyboard_input.just_pressed(KeyCode::KeyW) {
        return;
    }

    // Remove current piece so it doesn't collide with itself
    clear_active_from_board(&active, &mut board);

    // Calculate the next rotation state
    let next_rotation = (active.rotation + 1) % 4;
    let shape = shape_of(active.kind.clone(), next_rotation);
    let mut can_rotate = true;

    for (dx, dy) in shape.cells {
        let new_x = active.x + dx;
        let new_y = active.y + dy;

        // Check board bounds
        if new_x < 0 || new_x >= BOARD_WIDTH as i32 || new_y < 0 {
            can_rotate = false;
            break;
        }

        // Check for collisions with existing blocks
        if let Some(Cell::Filled(_)) = board.get(new_x, new_y) {
            can_rotate = false;
            break;
        }
    }

    // If rotation is valid, update the active piece
    if can_rotate {
        active.rotation = next_rotation;
    }

    // Draw the piece at its (possibly new) rotation
    place_active_on_board(&active, &mut board);
}