# Tet.rs

![Tetris](assets/tet.rs.png)

## Full Tetromino Implementation Plan

### 1. Finalize tetromino data model

- **Goal**: Have a clean representation of all 7 tetrominoes and their 4 rotation states, plus an `ActivePiece` that tracks the current falling tetromino.
- **Steps**:
- In [`src/tetrominoes.rs`](src/tetrominoes.rs), define static shape tables for all 7 tetromino kinds (`I, O, T, S, Z, J, L`) as arrays of relative `(dx, dy)` cell offsets for each of the 4 rotation states.
- Ensure `TetrominoKind` is `Copy + Clone + Debug` and `ActivePiece` is a Bevy `Resource` with fields: `kind`, `rotation: u8`, `x: i32`, `y: i32`.
- Implement `shape_of(kind, rotation)` to return a `TetrominoShape` referencing the correct offset table.
- Add a helper `spawn_new_at_top(kind: TetrominoKind) -> ActivePiece` that initializes `x` at the horizontal center and `y` just above or at the top rows.

### 2. Board + tetromino interaction helpers

- **Goal**: Provide reusable functions to query/modify the `Board` based on an `ActivePiece` plus a tentative movement or rotation.
- **Steps**:
- In [`src/tetrominoes.rs`](src/tetrominoes.rs) (or [`src/board.rs`](src/board.rs) if more appropriate), implement:
- `fn cells_for(active: &ActivePiece) -> impl Iterator<Item = (i32, i32)>` that yields absolute `(x, y)` positions for all blocks of the active piece.
- `fn can_place(active: &ActivePiece, board: &Board) -> bool` that checks bounds and ensures all target cells are either inside the board or above the top, and not colliding with `Cell::Filled`.
- `fn write_piece(active: &ActivePiece, board: &mut Board, color: Color)` that sets all relevant board cells to `Cell::Filled(color)`.
- `fn clear_piece(active: &ActivePiece, board: &mut Board)` that sets those cells back to `Cell::Empty`.
- Update the existing `place_active_on_board` / `clear_active_from_board` helpers (if present) to reuse these lower-level functions or replace them entirely to avoid duplication.

### 3. Spawning and falling logic

- **Goal**: Spawn a new random tetromino when the game enters `InGame`, and make it fall one row at a fixed interval.
- **Steps**:
- In [`src/game.rs`](src/game.rs), keep/adjust the existing `FallTimer` resource (fixed interval, e.g. 0.5s) and `fall_piece_system`.
- Add a `RandomTetromino` or use Bevy's `Random`/`rand` integration (or a simple custom PRNG) to pick a random `TetrominoKind` when spawning.
- Implement a `spawn_first_piece` system that:
- Creates a new `ActivePiece` at the top using `spawn_new_at_top(random_kind)`.
- Writes it into the `Board` using the helper from step 2.
- Inserts it as a `Resource`.
- Ensure this system runs on `OnEnter(AppState::InGame)` alongside `setup_ingame` and `setup_fall_timer`.

### 4. Basic gravity and collision/locking

- **Goal**: Each tick, try to move the active piece down; if blocked, lock it into the board and spawn a new piece.
- **Steps**:
- In `fall_piece_system` in [`src/game.rs`](src/game.rs):
- On each timer tick:
- Use the helper from step 2 to compute a tentative `ActivePiece` with `y - 1`.
- If `can_place` for the tentative piece is true:
  - Clear the current piece cells from the board.
  - Update the `ActivePiece` resource to the new `y`.
  - Write the piece back to the board.
- If `can_place` is false:
  - Treat this as a lock event: the current blocks stay on the board as part of the stack.
  - Trigger line clearing (step 5).
  - Spawn a new random `ActivePiece` at the top, and if `can_place` for it is false, this is game over (can be handled later or via a simple log/transition).

### 5. Line clearing on lock

- **Goal**: Remove any fully-filled rows after a piece locks and drop the rows above.
- **Steps**:
- In [`src/board.rs`](src/board.rs), add methods:
- `fn is_row_full(&self, y: usize) -> bool` that checks if a row has no `Cell::Empty`.
- `fn clear_full_rows(&mut self) -> u32` that:
- Scans all rows, collects full-row indices.
- For each full row, shifts all rows above it down by one and fills the top row with `Cell::Empty`.
- Returns the number of lines cleared (for scoring).
- Call `clear_full_rows` from the lock-handling branch in `fall_piece_system` after the piece is locked.
- Optionally, later integrate the returned count into a `Score` resource and update the UI text.

### 6. Horizontal movement and rotation controls

- **Goal**: Allow the player to move the active piece left/right and rotate it, with simple collision rules and no wall kicks.
- **Steps**:
- In [`src/game.rs`](src/game.rs), add a new `player_input_system` that:
- Reads keyboard input (e.g. `A/Left` = move left, `D/Right` = move right, `W/Up` or `K` = rotate, `S/Down` = soft drop).
- For each action:
- Build a tentative `ActivePiece` with updated `x` or `rotation`.
- Use `can_place` to see if the move is valid.
- If valid:
  - Clear the current piece from the board.
  - Apply the change to the `ActivePiece` resource.
  - Write the piece back to the board.
- Register `player_input_system` in `main.rs` inside the `Update` schedule gated by `in_state(AppState::InGame)` so it only runs during gameplay.

### 7. Visual polish and color mapping

- **Goal**: Make each tetromino type distinct and visually clear.
- **Steps**:
- In [`src/tetrominoes.rs`](src/tetrominoes.rs) or [`src/game.rs`](src/game.rs), define a helper `fn color_for(kind: TetrominoKind) -> Color` returning a distinct color per piece.
- Use `color_for(active.kind)` whenever calling `write_piece` / `place_active_on_board` so each active piece and its locked blocks keep their color.
- Optionally, adjust `sync_board` to use a darker or different color for empty cells so the colored blocks stand out.

### 8. Game-state edge cases (basic handling)

- **Goal**: Handle simple edge cases cleanly without implementing full guideline rules.
- **Steps**:
- When a new piece is spawned and `can_place` is false immediately, trigger a basic game-over condition (e.g., set `AppState` back to `MainMenu` or to a new `GameOver` state later).
- Ensure all helper functions treat coordinates above the board (`y >= BOARD_HEIGHT`) as allowed for spawning and falling, but disallow positions below 0 or outside `[0, BOARD_WIDTH)`.
- Add unit tests for `shape_of`, `can_place`, and `clear_full_rows` if desired to stabilize behavior.