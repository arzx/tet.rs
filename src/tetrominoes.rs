use crate::board::*;
use bevy::prelude::*;
use rand::Rng;
#[derive(Component, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TetrominoKind { I, O, T, S, Z, J, L }

pub struct TetrominoShape{
    pub cells: &'static [(i32,i32)]
}

const I_SHAPES: [[(i32, i32); 4]; 4] = [
    [(0, 0), (1, 0), (2, 0), (3, 0)],        // 0°
    [(1, -1), (1, 0), (1, 1), (1, 2)],       // 90°
    [(0, 1), (1, 1), (2, 1), (3, 1)],        // 180°
    [(2, -1), (2, 0), (2, 1), (2, 2)],       // 270°
];

const O_SHAPES: [[(i32, i32); 4]; 4] = [
    [(0, 0), (1, 0), (0, 1), (1, 1)],        // 0°
    [(0, 0), (1, 0), (0, 1), (1, 1)],        // 90° (same)
    [(0, 0), (1, 0), (0, 1), (1, 1)],        // 180°
    [(0, 0), (1, 0), (0, 1), (1, 1)],        // 270°
];

const T_SHAPES: [[(i32, i32); 4]; 4] = [
    [(0, 0), (1, 0), (2, 0), (1, 1)],        // 0°   _T_
    [(1, -1), (1, 0), (1, 1), (2, 0)],       // 90°   ⟂ right
    [(0, 1), (1, 1), (2, 1), (1, 0)],        // 180°
    [(1, -1), (1, 0), (1, 1), (0, 0)],       // 270°  ⟂ left
];

const S_SHAPES: [[(i32, i32); 4]; 4] = [
    [(1, 0), (2, 0), (0, 1), (1, 1)],        // 0°   horizontal S
    [(1, -1), (1, 0), (2, 0), (2, 1)],       // 90°
    [(1, 0), (2, 0), (0, 1), (1, 1)],        // 180°
    [(1, -1), (1, 0), (2, 0), (2, 1)],       // 270°
];

const Z_SHAPES: [[(i32, i32); 4]; 4] = [
    [(0, 0), (1, 0), (1, 1), (2, 1)],        // 0°   horizontal Z
    [(2, -1), (1, 0), (2, 0), (1, 1)],       // 90°
    [(0, 0), (1, 0), (1, 1), (2, 1)],        // 180°
    [(2, -1), (1, 0), (2, 0), (1, 1)],       // 270°
];

const J_SHAPES: [[(i32, i32); 4]; 4] = [
    [(0, 0), (0, 1), (1, 1), (2, 1)],        // 0°   ┘ flipped
    [(1, -1), (2, -1), (1, 0), (1, 1)],      // 90°
    [(0, 0), (1, 0), (2, 0), (2, 1)],        // 180°
    [(1, -1), (1, 0), (0, 1), (1, 1)],       // 270°
];

const L_SHAPES: [[(i32, i32); 4]; 4] = [
    [(2, 0), (0, 1), (1, 1), (2, 1)],        // 0°   └
    [(1, -1), (1, 0), (1, 1), (2, 1)],       // 90°
    [(0, 0), (1, 0), (2, 0), (0, 1)],        // 180°
    [(0, -1), (1, -1), (1, 0), (1, 1)],      // 270°
];

pub fn shape_of(kind: TetrominoKind, rotation: u8) -> TetrominoShape {
    let r = (rotation % 4) as usize;
    match kind {
        TetrominoKind::I => TetrominoShape { cells: &I_SHAPES[r] },
        TetrominoKind::O => TetrominoShape { cells: &O_SHAPES[r] },
        TetrominoKind::T => TetrominoShape { cells: &T_SHAPES[r] },
        TetrominoKind::S => TetrominoShape { cells: &S_SHAPES[r] },
        TetrominoKind::Z => TetrominoShape { cells: &Z_SHAPES[r] },
        TetrominoKind::J => TetrominoShape { cells: &J_SHAPES[r] },
        TetrominoKind::L => TetrominoShape { cells: &L_SHAPES[r] },
    }
}



#[derive(Resource)]
pub struct ActivePiece{
    pub kind: TetrominoKind,
    pub rotation: u8,
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl ActivePiece {
    pub fn spawn_new() -> Self {
        let mut rng = rand::thread_rng();

        let kinds = [
            TetrominoKind::I,
            TetrominoKind::O,
            TetrominoKind::T,
            TetrominoKind::S,
            TetrominoKind::Z,
            TetrominoKind::J,
            TetrominoKind::L,
        ];
        let kind = kinds[rng.gen_range(0..kinds.len())].clone();

        // random color
        let color = Color::srgb(
            rng.gen_range(0.2..1.0),
            rng.gen_range(0.2..1.0),
            rng.gen_range(0.2..1.0),
        );

        // random X so the 4‑wide bounding box stays on the board
        let x = rng.gen_range(0..=(BOARD_WIDTH as i32 - 4));

        Self {
            kind,
            rotation: 0,
            x,
            y: (BOARD_HEIGHT - 2) as i32,
            color,
        }
    }
}

pub fn place_active_on_board(active: &ActivePiece, board: &mut Board) {
    let shape = shape_of(active.kind.clone(), active.rotation);
    for (dx, dy) in shape.cells {
        board.set(active.x + dx, active.y + dy, Cell::Filled(active.color));
    }
}

pub fn clear_active_from_board(active: &ActivePiece, board: &mut Board) {
    let shape = shape_of(active.kind.clone(), active.rotation);
    for (dx, dy) in shape.cells {
        board.set(active.x + dx, active.y + dy, Cell::Empty);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_spawn_randomness() {
        let mut kinds = HashSet::new();
        for _ in 0..100 {
            let piece = ActivePiece::spawn_new();
            kinds.insert(piece.kind);
        }
        
        // We expect to see more than just 1 kind of piece after 100 spawns
        assert!(kinds.len() > 1, "Only spawned {:?} kinds: {:?}", kinds.len(), kinds);
        
        // Ideally we should see all 7, but let's be lenient and say at least 4 to account for extreme bad luck,
        // though with 100 iterations the probability of missing 3 types is vanishingly small.
        assert!(kinds.len() >= 4, "Distribution seems poor, only spawned: {:?}", kinds);
    }
}