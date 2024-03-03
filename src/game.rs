use std::io::{stdout, Write};

use macroquad::{miniquad::window::screen_size, prelude::*};

use crate::{
    board::Board,
    input::{clear_terminal, get_parsed_input, quit_terminal, Action},
};

pub const WIDTH: usize = 32;
pub const HEIGHT: usize = WIDTH;
pub const SCALE_FACTOR: f32 = WIDTH as f32 * 0.0008;

pub const EASY_MINES: f64 = 0.0625;
pub const MEDIUM_MINES: f64 = 0.125;
pub const HARD_MINES: f64 = 0.25;

#[derive(Debug, Clone, Copy)]
pub struct Game {
    board: Board<WIDTH, HEIGHT>,
    state: GameState,
    cell_index: (usize, usize),
    action: Action,
}
impl Game {
    pub fn new() -> Self {
        return Self {
            board: Board::random(MEDIUM_MINES),
            state: GameState::Playing,
            cell_index: (0, 0),
            action: Action::Cancel,
        };
    }

    /// Only to be called in execute_turn. must be called fourth
    pub fn execute_action(&mut self) {
        match self.action {
            Action::Reveal => {
                self.board[self.cell_index].reveal();
                if self.board[self.cell_index].is_mine() {
                    self.state = GameState::GameOver;
                }
            }
            Action::Flag => self.board[self.cell_index].flag(),
            Action::Unflag => self.board[self.cell_index].unflag(),
            Action::Cancel => (),
        };
    }

    pub fn set_action(&mut self, action: Action) {
        self.action = action;
    }

    pub fn set_cell_index(&mut self, cell_index: (usize, usize)) {
        if self.board.get_cell(cell_index).is_some() {
            self.cell_index = cell_index;
        }
    }
}
/// macroquad methods
impl Game {
    pub fn update(&mut self) -> GameState {
        if let GameState::Playing = self.state {
            self.handle_input();
            self.execute_action();
        };
        return self.state;
    }

    fn cell_size() -> f32 {
        screen_width().min(screen_height()) * SCALE_FACTOR
    }

    fn cell_boundaries() -> [[Rect; WIDTH]; HEIGHT] {
        let mut boundaries = [[Rect::default(); WIDTH]; HEIGHT];

        let cell_size: f32 = Self::cell_size();

        let window_origin = Vec2::from(screen_size()) / 2.0;

        let board_position =
            window_origin - (Vec2::new(WIDTH as f32, HEIGHT as f32) * cell_size) / 2.0;

        for row_index in 0..HEIGHT {
            for column_index in 0..WIDTH {
                
                let cell_position =
                    board_position + (cell_size * Vec2::new(row_index as f32, column_index as f32));

                let cell_boundary =
                    Rect::new(cell_position.x, cell_position.y, cell_size, cell_size);

                boundaries[row_index][column_index] = cell_boundary;
            }
        }

        return boundaries;
    }

    pub fn draw(&mut self) {
        let cell_boundaries = Self::cell_boundaries();
        let cell_size: f32 = Self::cell_size();
        let border_size = cell_size * 0.05;

        for (row_index, row) in cell_boundaries.into_iter().enumerate() {
            for (column_index, boundary) in row.into_iter().enumerate() {
                let color = self
                    .board
                    .get_cell((row_index, column_index))
                    .and_then(|cell| Some(cell.color()))
                    .unwrap_or(WHITE);

                // draw border
                draw_rectangle(boundary.x, boundary.y, boundary.w, boundary.h, BLACK);

                // draw interior
                draw_rectangle(
                    boundary.x + border_size,
                    boundary.y + border_size,
                    boundary.w - 2.0 * border_size,
                    boundary.h - 2.0 * border_size,
                    color,
                );
            }
        }
    }

    fn handle_input(&mut self) {
        let mut click = false;
        
        if is_mouse_button_released(MouseButton::Left) {
            self.action = Action::Reveal;
            click = true;
        }
        if is_mouse_button_released(MouseButton::Right) {
            match self.action {
                Action::Flag => self.action = Action::Unflag,
                Action::Unflag => self.action = Action::Flag,
                _ => {},
            };
            click = true;
        }

        if click {

            for (row_index, row) in Self::cell_boundaries().into_iter().enumerate() {
                for (column_index, boundary) in row.into_iter().enumerate() {
                    
                    if boundary.contains(mouse_position().into()) {
                        
                        if let Some(cell) = self.board.get_cell_mut((row_index, column_index)) {
                            self.set_cell_index((row_index, column_index));
                            self.execute_action();                      
                        }

                        break;
                    }
                }
            }
        }
    }

    fn handle_game_over(&mut self) {

    }
}
/// Terminal methods
impl Game {
    pub fn update_terminal(&mut self) -> Result<GameState, std::io::Error> {
        if let GameState::Playing = self.state {
            self.print_board()?;
            self.get_cell_index_terminal()?;
            self.get_action_terminal()?;
            self.execute_action();
            self.handle_game_over_terminal()?;
        }
        return Ok(self.state);
    }

    /// Only to be called in execute_turn. must be called first
    fn print_board(&self) -> Result<(), std::io::Error> {
        clear_terminal()?;
        writeln!(stdout(), "{}\n", self.board)?;
        return Ok(());
    }

    /// Only to be called in execute_turn. must be called second
    fn get_cell_index_terminal(&mut self) -> Result<(), std::io::Error> {
        loop {
            // allow user to select a cell
            let row_index = get_parsed_input("Select a cell\nPlease enter a row number: ")?;
            let column_index = get_parsed_input("Please enter a column number: ")?;

            // ensure the user entered a valid cell
            if self.board.get_cell((row_index, column_index)).is_some() {
                self.cell_index = (row_index, column_index);
                return Ok(());
            }

            println!(
                "\nThat cell is out of {}x{} bounds. Try again\n",
                self.board.width(),
                self.board.height()
            );
        }
    }

    /// Only to be called in execute_turn. must be called third
    pub fn get_action_terminal(&mut self) -> Result<(), std::io::Error> {
        self.action =
            get_parsed_input("\nSelect an action for this cell\nReveal\nFlag\nUnflag\nCancel\n")?;
        return Ok(());
    }

    pub fn handle_game_over_terminal(&mut self) -> Result<(), std::io::Error> {
        clear_terminal()?;
        writeln!(
            stdout(),
            "YOU REVEALED A MINE\nGAME OVER\n{}",
            self.board.clone_revealed()
        )?;
        self.state = if quit_terminal()? {
            GameState::Quit
        } else {
            GameState::Playing
        };
        return Ok(());
    }
}

/// A marker type to signify a game over
#[derive(Debug, Clone, Copy)]
pub enum GameState {
    GameOver,
    Playing,
    Quit,
}
impl GameState {
    pub fn is_game_over(&self) -> bool {
        return if let GameState::GameOver = self {
            true
        } else {
            false
        };
    }
}
