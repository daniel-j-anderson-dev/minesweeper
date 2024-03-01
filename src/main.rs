// modules
mod board;
mod cell;
mod input;

use crate::{
    board::Board,
    input::{clear_terminal, get_parsed_input, quit, CellAction},
};

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut board = Board::<WIDTH, HEIGHT>::random(0.20);

    loop {
        clear_terminal();
        println!("\n{}", board);

        // allow user to select a cell
        let row_index = get_parsed_input("Select a cell\nPlease enter a row number: ")?;
        let column_index = get_parsed_input("Please enter a column number: ")?;

        // ensure the user entered a valid cell
        let cell = match board.get_cell_mut(row_index, column_index) {
            Some(cell) => cell,
            None => {
                println!(
                    "\n({},{}) is not on the board.\nNote: row/column numbers start at 0",
                    row_index, column_index
                );
                continue;
            }
        };

        // allow the user to choose an action with that cell
        let cell_action =
            get_parsed_input("\nSelect an action for this cell\nReveal\nFlag\nUnflag\nCancel\n")?;

        // perform cell action
        match cell_action {
            CellAction::Reveal => {
                // reveal the cell
                cell.reveal();

                // if the cell is a mine then game over
                if cell.is_mine() {
                    clear_terminal();
                    println!(
                        "\nYOU REVEALED A MINE!\nGAME OVER\n{}\n",
                        board.cells_revealed()
                    );

                    // Reset the game if the user doesn't want to quit
                    if quit()? {
                        break;
                    } else {
                        board = Board::<10, 10>::random(0.20);
                        continue;
                    }
                }
            }
            CellAction::Flag => cell.flag(),
            CellAction::Unflag => cell.unflag(),
            CellAction::Cancel => continue,
        };
    }

    return Ok(());
}
