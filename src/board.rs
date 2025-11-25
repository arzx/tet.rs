use bevy::prelude::*;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Filled(Color),
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            cells: [[Cell::Empty; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }
}

#[derive(Resource)]
pub struct Board {
    pub cells: [[Cell; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn get(&self, x: i32, y: i32) -> Option<Cell> {
        if x < 0 || y < 0 {
            return None;
        }
        let (x, y) = (x as usize, y as usize);
        if x >= BOARD_WIDTH || y >= BOARD_HEIGHT {
            return None;
        }
        Some(self.cells[y][x])
    }

    pub fn set(&mut self, x: i32, y: i32, cell: Cell) {
        if let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(y)) {
            if x < BOARD_WIDTH && y < BOARD_HEIGHT {
                self.cells[y][x] = cell;
            }
        }
    }
}
