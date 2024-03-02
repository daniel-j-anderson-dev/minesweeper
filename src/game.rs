use std::io::{stdout, Write};

use macroquad::{miniquad::window::screen_size, prelude::*};

use crate::{board::Board, input::{clear_terminal, get_parsed_input, Action}};

pub const WIDTH: usize = 32;
pub const HEIGHT: usize = WIDTH;

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

    pub fn execute_turn(&mut self) -> GameState {
        return match self.state {
            GameState::GameOver =>  GameState::GameOver,
            GameState::Playing => {
                self.handle_input();
                self.execute_action();
                self.state
            },
        };
    }

    pub fn execute_terminal_turn(&mut self) -> Result<GameState, std::io::Error> {
        return Ok(match self.state {
            GameState::GameOver => GameState::GameOver,
            GameState::Playing => {
                self.display_board_terminal()?;
                self.get_cell_index_terminal()?;
                self.get_action_terminal()?;
                self.execute_action();
                self.state
            },
        });
    }

    /// Only to be called in execute_turn. must be called first
    fn display_board_terminal(&self) -> Result<(), std::io::Error> {
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
        self.cell_index = cell_index;
    }

    pub fn draw(&mut self) {
        let size: f32 = screen_width().min(screen_height()) * 0.027;
        let screen_origin = Vec2::from(screen_size()) / 2.0;
        let board_position = screen_origin - (Vec2::new(WIDTH as f32, HEIGHT as f32) * size) / 2.0;
        let border_size = size / 10.0;

        for row_index in 0..HEIGHT {
            for column_index in 0..WIDTH {
                let cell_position = Vec2::new(size * row_index as f32, size * column_index as f32) + board_position;
                
                let cell_boundary = Rect::new(cell_position.x, cell_position.y, size, size);
                
                // draw border
                draw_rectangle(
                    cell_boundary.x,
                    cell_boundary.y,
                    cell_boundary.w,
                    cell_boundary.h,
                    GRAY
                );
    
                // draw interior
                draw_rectangle(
                    cell_boundary.x + border_size,
                    cell_boundary.y + border_size,
                    cell_boundary.w - 2.0 * border_size,
                    cell_boundary.h - 2.0 * border_size,
                    LIGHTGRAY
                );
            }
        }
    }
    
    fn handle_input(&mut self) {
    }
}

/// A marker type to signify a game over
#[derive(Debug, Clone, Copy)]
pub enum GameState {
    GameOver,
    Playing,
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
