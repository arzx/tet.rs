use crate::board::*;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub enum TetrominoKind { I, O, T, S, Z, J, L }

pub struct TetrominoShape{
    pub cells: &'static [(i32,i32)]
}

const I_SHAPES: [[(i32, i32); 4]; 4] = [
    [(0, 0), (1, 0), (2, 0), (3, 0)], // 0°
    [(1, -1), (1, 0), (1, 1), (1, 2)], // 90°
    [(0, 1), (1, 1), (2, 1), (3, 1)], // 180°
    [(2, -1), (2, 0), (2, 1), (2, 2)], // 270°
];

pub fn shape_of(kind: TetrominoKind, rotation: u8) -> TetrominoShape{
    let r = (rotation % 4) as usize;
    match kind {
        TetrominoKind::I => TetrominoShape {cells: &I_SHAPES[r]},
        _ => TetrominoShape {cells: &I_SHAPES[0]},
    }
}


#[derive(Resource)]
pub struct ActivePiece{
    pub kind: TetrominoKind,
    pub rotation: u8,
    pub x: i32,
    pub y: i32,
}

impl ActivePiece {
    pub fn spawn_new() -> Self {
        Self {
            kind: TetrominoKind::I,
            rotation: 0,
            // center-ish at top
            x: (BOARD_WIDTH / 2 - 2) as i32,
            y: (BOARD_HEIGHT - 1) as i32,
        }
    }
}

pub fn place_active_on_board(active: &ActivePiece, board: &mut Board, color: Color) {
    let shape = shape_of(active.kind.clone(), active.rotation);
    for (dx, dy) in shape.cells {
        board.set(active.x + dx, active.y + dy, Cell::Filled(color));
    }
}

pub fn clear_active_from_board(active: &ActivePiece, board: &mut Board) {
    let shape = shape_of(active.kind.clone(), active.rotation);
    for (dx, dy) in shape.cells {
        board.set(active.x + dx, active.y + dy, Cell::Empty);
    }
}