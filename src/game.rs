use std::io::{stdout, Write};

use crate::{
    board::Board,
    input::{clear_terminal, get_parsed_input, Action},
};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 10;

pub const EASY_MINES: f64 = 0.0625;
pub const MEDIUM_MINES: f64 = 0.125;
pub const HARD_MINES: f64 = 0.25;

pub struct Game {
    board: Board<WIDTH, HEIGHT>,
    is_game_over: bool,
    cell_index: (usize, usize),
    action: Action,
}
impl Game {
    pub fn new() -> Self {
        return Self {
            board: Board::random(MEDIUM_MINES),
            is_game_over: false,
            cell_index: (0, 0),
            action: Action::Cancel,
        };
    }

    pub fn execute_turn(&mut self) -> Result<GameState, std::io::Error> {
        self.display_board()?;
        self.get_cell_index()?;
        self.get_action()?;
        self.execute_action()?;
        return self.handle_game_over();
    }

    fn display_board(&self) -> Result<(), std::io::Error> {
        clear_terminal()?;
        writeln!(stdout(), "{}\n", self.board)?;
        return Ok(());
    }

    fn get_cell_index(&mut self) -> Result<(), std::io::Error> {
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

    pub fn get_action(&mut self) -> Result<(), std::io::Error> {
        self.action =
            get_parsed_input("\nSelect an action for this cell\nReveal\nFlag\nUnflag\nCancel\n")?;
        return Ok(());
    }

    pub fn execute_action(&mut self) -> Result<(), std::io::Error> {
        match self.action {
            Action::Reveal => {
                self.board[self.cell_index].reveal();
                if self.board[self.cell_index].is_mine() {
                    self.is_game_over = true;
                }
            }
            Action::Flag => self.board[self.cell_index].flag(),
            Action::Unflag => self.board[self.cell_index].unflag(),
            Action::Cancel => (),
        };
        return Ok(());
    }

    pub fn handle_game_over(&mut self) -> Result<GameState, std::io::Error> {
        return if self.action.is_reveal() && self.board[self.cell_index].is_mine() {
            clear_terminal()?;
            writeln!(stdout(), "\nYou revealed a mine!\nGAME OVER\n{}", self.board.clone_revealed())?;
            Ok(GameState::GameOver)
        } else {
            Ok(GameState::Playing)
        };
    }
}

/// A marker type to signify a game over
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