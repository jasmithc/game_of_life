// Import necessary modules and dependencies
use ggez::{
    event::{self, EventHandler},
    graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect},
    input::mouse,
    Context, ContextBuilder, GameResult,
};
use rand::Rng;

// Constants for grid and screen dimensions
const GRID_WIDTH: isize = 32;
const GRID_HEIGHT: isize = 32;
const GRID_CELL_SIZE: isize = 20;
const SCREEN_SIZE: (f32, f32) = (
    GRID_WIDTH as f32 * GRID_CELL_SIZE as f32,
    GRID_HEIGHT as f32 * GRID_CELL_SIZE as f32,
);
const TARGET_FPS: u32 = 15;

// GameState struct to maintain the game state
struct GameState {
    board: Board,
    mouse_down: bool,
}

impl GameState {
    // Initialize a new game state with a randomized board
    fn new() -> GameState {
        let mut game = GameState {
            board: Board::new(GRID_WIDTH, GRID_HEIGHT),
            mouse_down: false,
        };
        game.randomize();
        game
    }

    // Randomize the board
    fn randomize(&mut self) {
        self.board.randomize();
    }

    // Handle mouse events to "spawn" cells
    fn handle_mouse(&mut self, x: f32, y: f32) {
        //Scale the mouse coordinate to the grid coordinates.
        //This is necessary because the mouse coordinates are in screen coordinates,
        //not grid coordinates.
        //Without this scaling, I was only able to spawn cells neaer the top left corner

        let grid_x = (x / GRID_CELL_SIZE as f32) as isize;
        let grid_y = (y / GRID_CELL_SIZE as f32) as isize;

        if let Some(cell) = self.board.get_cell_mut(grid_x, grid_y) {
            cell.alive = true;
        }
    }
}

impl EventHandler for GameState {
    // Update the game state
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(TARGET_FPS) {
            self.board.update();
        }
        Ok(())
    }

    // Draw the game board
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        for cell in &self.board.cells {
            if cell.alive {
                let rect = Rect::new(
                    (cell.pos_x * GRID_CELL_SIZE) as f32,
                    (cell.pos_y * GRID_CELL_SIZE) as f32,
                    GRID_CELL_SIZE as f32,
                    GRID_CELL_SIZE as f32,
                );

                let square = Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::BLACK)?;
                canvas.draw(&square, DrawParam::default());
            }
        }

        canvas.finish(ctx)?;
        ggez::timer::yield_now();
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

// Board struct representing the game board
struct Board {
    cells: Vec<Cell>,
    width: isize,
    height: isize,
}

impl Board {
    // Create a new board with the given dimensions
    fn new(width: isize, height: isize) -> Board {
        let mut cells = vec![];
        for y in 0..height {
            for x in 0..width {
                cells.push(Cell::new(x, y));
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
        for cell in self.cells.iter_mut() {
            cell.alive = rand::thread_rng().gen_range(0..=100) < 50;
        }
    }

    // Get a mutable reference to a cell at the given coordinates
    fn get_cell_mut(&mut self, x: isize, y: isize) -> Option<&mut Cell> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(&mut self.cells[((y * self.width) + x) as usize])
        } else {
            None
        }
    }

    fn get_cell(&self, x: isize, y: isize) -> Option<&Cell> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some(&self.cells[((y * self.width) + x) as usize])
        } else {
            None
        }
    }

    // Update the board based on the rules of the game
    fn update(&mut self) {
        let mut new_cells = self.cells.clone();

        for cell in &mut new_cells {
            let alive_neighbors = self.count_alive_neighbors(cell.pos_x, cell.pos_y);

            match (cell.alive, alive_neighbors) {
                (true, 2) | (true, 3) => cell.alive = true,
                (true, _) => cell.alive = false,
                (false, 3) => cell.alive = true,
                _ => cell.alive = false,
            }
        }

        self.cells = new_cells;
    }

    // Count the number of alive neighbors for a cell
    fn count_alive_neighbors(&mut self, x: isize, y: isize) -> usize {
        let mut count = 0;

        for ny in y - 1..=y + 1 {
            for nx in x - 1..=x + 1 {
                if nx == x && ny == y {
                    continue;
                }

                if let Some(cell) = self.get_cell(nx, ny) {
                    if cell.alive {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

// Cell struct representing an individual cell on the board
#[derive(Debug, Clone, Copy)]
struct Cell {
    alive: bool,
    pos_x: isize,
    pos_y: isize,
}

impl Cell {
    // Create a new cell at the given position
    fn new(pos_x: isize, pos_y: isize) -> Cell {
        Cell {
            alive: false,
            pos_x,
            pos_y,
        }
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
