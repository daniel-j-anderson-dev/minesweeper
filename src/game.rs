use crate::{board::Board, input::{get_parsed_input, Action}};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 10;

pub const EASY_MINES: f64 = 0.1;
pub const MEDIUM_MINES: f64 = 0.25;
pub const HARD_MINES: f64 = 0.5;

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
            
        }
    }

    pub fn update(&mut self) -> Result<(), std::io::Error> {
        self.get_cell_index()?;

        return Ok(());
    }

    pub fn get_cell_index(&mut self) -> Result<(), std::io::Error> {
        loop {
            // allow user to select a cell
            let row_index = get_parsed_input("Select a cell\nPlease enter a row number: ")?;
            let column_index = get_parsed_input("Please enter a column number: ")?;
    
            // ensure the user entered a valid cell
            if self.board.get_cell_mut(row_index, column_index).is_none() {
                self.cell_index = (row_index, column_index);
                return Ok(())
            }

            println!("\nInvalid input\n")
        }
    }
}