// Import necessary modules and dependencies
use ggez::{
    event::{self, EventHandler},
    graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect},
    input::mouse,
    Context, ContextBuilder, GameResult,
};
use rand::Rng;
use std::time::{Duration, Instant};

// Constants for grid and screen dimensions
const GRID_WIDTH: u32 = 200;
const GRID_HEIGHT: u32 = 200;
const GRID_CELL_SIZE: i32 = 8;
const SCREEN_SIZE: (f32, f32) = (
    GRID_WIDTH as f32 * GRID_CELL_SIZE as f32,
    GRID_HEIGHT as f32 * GRID_CELL_SIZE as f32,
);
const TARGET_FPS: f64 = 90.0;

// Utility functions

// Moved this here as it was used in a few places.
// Just calculates the x and y coordinates from the given index.
fn get_coordinates(i: i32) -> (i32, i32) {
    let x: i32 = i % GRID_WIDTH as i32;
    let y: i32 = i / GRID_WIDTH as i32;
    (x, y)
}

// Cell struct representing an individual cell on the board
#[derive(Debug, Clone, Copy)]
struct Cell {
    alive: bool,
}

impl Cell {
    // Create a new cell at the given position
    fn new() -> Cell {
        Cell { alive: false }
    }
}

// Board struct representing the game board
struct Board {
    cells: Vec<Cell>,
    width: u32,
    height: u32,
}

impl Board {
    // Create a new board with the given dimensions
    fn new(width: u32, height: u32) -> Board {
        let mut cells = vec![];
        for _ in 0..height {
            for _ in 0..width {
                cells.push(Cell::new());
            }
        }
        Board {
            cells,
            width,
            height,
        }
    }

    // Randomize the board's cells
    fn randomize(&mut self) {
        for (i, cell) in self.cells.iter_mut().enumerate() {
            // cell.alive = rand::thread_rng().gen_range(0..=100) < 35;

            // Cool pattern. Becomes stable at around 500 cycles.
            cell.alive = i % 3 == 0;
        }
    }

    // Get a mutable reference to a cell at the given coordinates
    fn get_cell_mut(&mut self, x: u32, y: u32) -> Option<&mut Cell> {
        if x as isize >= 0 && x < self.width && y as isize >= 0 && y < self.height {
            Some(&mut self.cells[((y * self.width) + x) as usize])
        } else {
            None
        }
    }

    fn get_cell(&self, x: u32, y: u32) -> Option<&Cell> {
        if x as isize >= 0 && x < self.width && y as isize >= 0 && y < self.height {
            Some(&self.cells[((y * self.width) + x) as usize])
        } else {
            None
        }
    }

    // Update the board based on the rules of the game.
    fn update(&mut self, future_board: &mut Board) {
        for i in 0..self.cells.len() {
            let cell = &self.cells[i];
            let alive_neighbors = &self.count_alive_neighbors(i as i32);

            future_board.cells[i].alive = match (cell.alive, alive_neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            }
        }

        // self.cells.clone_from_slice(&future_board.cells);
    }

    // Count the number of alive neighbors for a cell
    fn count_alive_neighbors(&self, i: i32) -> usize {
        let (x, y) = get_coordinates(i);
        let mut count = 0;

        for ny in y - 1..=y + 1 {
            for nx in x - 1..=x + 1 {
                if nx == x && ny == y {
                    continue;
                }

                if let Some(cell) = self.get_cell(nx as u32, ny as u32) {
                    if cell.alive {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

// GameState struct to maintain the game state
struct GameState {
    board_1: Board,
    board_2: Board,
    mouse_down: bool,
    cycle: u32,                // Track the current cycle
    last_update: Instant,      // Track the last update time
    update_interval: Duration, // Set the update interval
}

impl GameState {
    // Initialize a new game state with a randomized board
    fn new() -> GameState {
        let mut game = GameState {
            board_1: Board::new(GRID_WIDTH, GRID_HEIGHT),
            board_2: Board::new(GRID_WIDTH, GRID_HEIGHT),
            mouse_down: false,
            cycle: 0,
            last_update: Instant::now(),
            // I think this should be 60hz tick rate, but I'm not sure.
            update_interval: Duration::from_secs_f32(0.1 / 60.0),
        };
        game.randomize();
        game
    }

    // Randomize the board
    fn randomize(&mut self) {
        self.board_1.randomize();
    }

    // Handle mouse events to "spawn" cells
    fn handle_mouse(&mut self, x: f32, y: f32) {
        //Scale the mouse coordinate to the grid coordinates.
        //This is necessary because the mouse coordinates are in screen coordinates,
        //not grid coordinates.
        //Without this scaling, I was only able to spawn cells neaer the top left corner

        let grid_x = (x / GRID_CELL_SIZE as f32) as u32;
        let grid_y = (y / GRID_CELL_SIZE as f32) as u32;

        if let Some(cell) = match self.cycle % 2 {
            0 => self.board_1.get_cell_mut(grid_x, grid_y),
            _ => self.board_2.get_cell_mut(grid_x, grid_y),
        } {
            cell.alive = true;
        }
    }
}

impl EventHandler for GameState {
    // Update the game state
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Check if it's time to update the board
        if self.last_update.elapsed() >= self.update_interval {
            self.last_update = Instant::now(); // Reset the timer

            let (current_board, future_board) = match self.cycle % 2 {
                0 => (&mut self.board_1, &mut self.board_2),
                _ => (&mut self.board_2, &mut self.board_1),
            };
            current_board.update(future_board);

            self.cycle += 1;

            println!(
                "Cycle {}: Update took {:?}",
                self.cycle,
                self.last_update.elapsed()
            );
        }

        Ok(())
    }

    // Draw the game board
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let frame_duration = Duration::from_secs_f64(1.0 / TARGET_FPS);

        let start_time = Instant::now();

        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        for i in 0..self.board_1.cells.len() {
            let cell = self.board_1.cells[i];
            if cell.alive {
                let (x, y) = get_coordinates(i as i32);
                let rect = Rect::new(
                    (x * GRID_CELL_SIZE) as f32,
                    (y * GRID_CELL_SIZE) as f32,
                    GRID_CELL_SIZE as f32,
                    GRID_CELL_SIZE as f32,
                );

                let square = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::BLACK)?;
                canvas.draw(&square, DrawParam::default());
            }
        }

        canvas.finish(ctx)?;

        // Calculate the time taken to render the frame
        let elapsed = start_time.elapsed();

        // Sleep the remaining time to achieve the target frame rate
        if elapsed < frame_duration {
            ggez::timer::sleep(frame_duration - elapsed);
        }

        Ok(())
    }

    // Handle mouse button down event
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: mouse::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        if button == mouse::MouseButton::Left {
            self.mouse_down = true;
            self.handle_mouse(x, y);
        }
        Ok(())
    }

    // Handle mouse button up event
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: mouse::MouseButton,
        _: f32,
        _: f32,
    ) -> GameResult {
        if button == mouse::MouseButton::Left {
            self.mouse_down = false;
        }
        Ok(())
    }

    // Handle mouse motion event
    fn mouse_motion_event(
        &mut self,
        _: &mut Context,
        x: f32,
        y: f32,
        _: f32,
        _: f32,
    ) -> GameResult {
        if self.mouse_down {
            self.handle_mouse(x, y);
        }
        Ok(())
    }
}

// Main function to start the game
fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("game_of_life", "JASC")
        .window_setup(ggez::conf::WindowSetup::default().title("Game of Life"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = GameState::new();
    event::run(ctx, event_loop, state)
}
