use rand::Rng;
use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
    str::FromStr,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut board = Board::<10, 10>::random(0.20);

    loop {
        clear_terminal();
        println!("\n{}x{} Board:\n{}", board.width(), board.height(), board);

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
        let cell_action: CellAction =
            get_parsed_input("\nSelect an action for this cell\nReveal\nFlag\nUnflag\nCancel\n")?;

        // perform cell action
        match cell_action {
            CellAction::Reveal => {
                // reveal the cell
                cell.reveal();

                // if the cell is a mine then game over
                if cell.is_mine {
                    clear_terminal();
                    for row in board.cells_mut() {
                        for cell in row {
                            cell.reveal()
                        }
                    };
                    println!("\nYOU REVEALED A MINE!\nGAME OVER\n{}\n", board);

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

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn quit() -> Result<bool, std::io::Error> {
    return match get_input("play again? (Enter yes to play again)\n")?
        .to_lowercase()
        .as_str()
    {
        "y" | "yes" => Ok(false),
        _ => Ok(true),
    };
}

#[derive(Debug)]
pub enum CellAction {
    Reveal,
    Flag,
    Unflag,
    Cancel,
}
impl FromStr for CellAction {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.to_lowercase().as_str() {
            "r" | "reveal" => Ok(CellAction::Reveal),
            "f" | "flag" => Ok(CellAction::Flag),
            "u" | "unflag" => Ok(CellAction::Unflag),
            "c" | "cancel" => Ok(CellAction::Cancel),
            invalid => Err(format!("{} is not a valid cell action.\n either use the first letter or type the whole action", invalid).into()),
        };
    }
}

/// <b> This function will call [get_input] forever until the [String] returned can be `parsed` into a `T`. </b>
/// # Generics
/// - `T`: This type must implement [FromStr] meaning it can be parsed.
///   - The [FromStr::Err] type associated with `T` must also implement the [std::fmt::Display] trait to be shown to the user
/// # Errors
/// - When [get_input] fails
pub fn get_parsed_input<T>(prompt: &str) -> Result<T, std::io::Error>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    loop {
        let input = get_input(prompt)?; // get input

        // attempt to parse input
        match input.parse() {
            Ok(parsed_input) => return Ok(parsed_input), // if the input could be parsed return it
            Err(parse_error) => println!("\nInvalid input\n{}\n", parse_error), // otherwise print an error and continue the loop
        }
    }
}

/// This function will display the `prompt` to the user using `standard output stream` ([std::io::Stdout]). <br>
/// A line of input is read from `standard input stream` ([std::io::Stdin]) and returned.
/// - Trailing and leading whitespace is trimmed from the input line.
/// # Errors
/// - If cannot write `prompt` to `standard output stream`
/// - If cannot flush `standard output stream`
/// - If cannot read input from `standard input stream`
pub fn get_input(prompt: &str) -> Result<String, std::io::Error> {
    // prompt the user
    stdout().write(prompt.as_bytes())?; // write the prompt to `stdout`
    stdout().flush()?; // flush the standard output stream (ensure all data reaches its destination ie the terminal)

    // read a line of input
    let mut input = String::new(); // create a [String] to hold user input
    stdin().read_line(&mut input)?; // read a line from `stdin` into `input`
    let input = input.trim().to_string(); // shadow `input` with a clone that doesn't include leading or trailing whitespace

    return Ok(input);
}

#[derive(Clone, Copy)]
/// contains the information pertaining to a cell
struct Cell {
    is_mine: bool,
    /// number of mines surrounding this [Cell]
    local_mines: usize,
    /// represents if the user has picked to reveal contents of [Cell]
    is_revealed: bool,
    /// represents if the user has flagged the [Cell]
    is_flagged: bool,
}
impl Display for Cell {
    /// display as mine as "*". If empty, displays the number of mines in cells around it
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_revealed {
            if self.is_mine {
                write!(f, "*")?;
            } else {
                write!(f, "{}", self.local_mines)?;
            }
        } else if self.is_flagged {
            write!(f, "⚑")?;
        } else {
            write!(f, "#")?;
        }
        return Ok(());
    }
}
impl Cell {
    /// a clear cell used for a default value
    pub const CLEAR: Self = Self {
        local_mines: 0,
        is_mine: false,
        is_revealed: false,
        is_flagged: false,
    };
    /// used to generate a random cell
    pub fn random(is_mine_percentage: f64) -> Self {
        let mut rng = rand::thread_rng(); // thread specific random number generator
        return Self {
            is_mine: rng.gen_bool(is_mine_percentage),
            local_mines: 0,
            is_revealed: false,
            is_flagged: false,
        };
    }
    pub fn reveal(&mut self) {
        self.is_revealed = true;
    }
    pub fn flag(&mut self) {
        self.is_flagged = true;
    }
    pub fn unflag(&mut self) {
        self.is_flagged = false;
    }
}

/// A 2 dimensional board of `WIDTH` x `HEIGHT` [Cell]s in area.
struct Board<const WIDTH: usize, const HEIGHT: usize> {
    cells: [[Cell; WIDTH]; HEIGHT],
}
impl<const W: usize, const H: usize> Display for Board<W, H> {
    /// displays a board as a grid. rows delimited by new line, cells delimited by a space
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.cells.iter() {
            for element in row.iter() {
                write!(f, "{} ", element)?;
            }
            write!(f, "\n")?;
        }
        return Ok(());
    }
}
impl<const W: usize, const H: usize> Board<W, H> {
    /// Initialize the minesweeper board with random true/false
    pub fn random(is_mine_percentage: f64) -> Self {
        // create cells that have a 50% chance of being a mine
        let cells = Board::random_cells(is_mine_percentage);
        let cells = Board::initialize_local_mines(cells);

        return Board { cells: cells };
    }

    /// Create a `W`x`H` 2d array of [Cell]s that each have a `is_mine_percentage` of being a mine
    fn random_cells(is_mine_percentage: f64) -> [[Cell; W]; H] {
        let mut cells = [[Cell::CLEAR; W]; H];

        for row in cells.iter_mut() {
            for cell in row.iter_mut() {
                *cell = Cell::random(is_mine_percentage);
            }
        }

        return cells;
    }

    /// Initializes all of the `cell`s `local_mines` field.
    fn initialize_local_mines(mut cells: [[Cell; W]; H]) -> [[Cell; W]; H] {
        // I didn't use an iterator bc I couldn't wrap my head around the logic for this given scenario...
        // It needed to be able to iterate across local cells within the board while also iterating across the entire board
        for row in 0..W {
            for column in 0..H {
                // count the mines local to this cell
                let local_mine_count = Self::count_local_mines(&cells, row, column);

                // set the local mine count for the given cell, based on the count accumulated
                cells[row][column].local_mines = local_mine_count;
            }
        }

        return cells;
    }

    /// count the number of [Cell]s that are mines surrounding a [Cell] at the specified indices
    fn count_local_mines(cells: &[[Cell; W]; H], row_index: usize, column_index: usize) -> usize {
        let mut local_mine_count = 0;

        // if the cell in question is not a mine
        if !cells[row_index][column_index].is_mine {
            // list indices of all neighboring cells
            let neighbor_indices = [
                (row_index as isize - 1, column_index as isize - 1),
                (row_index as isize - 1, column_index as isize - 0),
                (row_index as isize - 1, column_index as isize + 1),
                (row_index as isize - 0, column_index as isize - 1),
                (row_index as isize + 0, column_index as isize + 1),
                (row_index as isize + 1, column_index as isize - 1),
                (row_index as isize + 1, column_index as isize + 0),
                (row_index as isize + 1, column_index as isize + 1),
            ];

            for (neighbor_row, neighbor_column) in neighbor_indices {
                // if the neighbor is in bounds
                if neighbor_row >= 0
                    && neighbor_column >= 0
                    && neighbor_row < H as isize
                    && neighbor_column < W as isize
                {
                    // Increment the local mine count
                    if cells[neighbor_row as usize][neighbor_column as usize].is_mine {
                        local_mine_count += 1;
                    }
                }
            }
        }

        return local_mine_count;
    }

    pub const fn width(&self) -> usize {
        return W;
    }
    pub const fn height(&self) -> usize {
        return H;
    }

    /// This function returns a reference to a specified cell if the index is valid
    pub fn get_cell(&self, row_index: usize, column_index: usize) -> Option<&Cell> {
        return self
            .cells
            .get(row_index)
            .and_then(|row| row.get(column_index));
    }
    /// This function returns a mutable reference to a specified cell if the index is valid
    pub fn get_cell_mut(&mut self, row_index: usize, column_index: usize) -> Option<&mut Cell> {
        return self
            .cells
            .get_mut(row_index)
            .and_then(|row| row.get_mut(column_index));
    }
    pub fn cells(&self) -> &[[Cell; W]; H] {
        return &self.cells;
    }
    pub fn cells_mut(&mut self) -> &mut [[Cell; W]; H] {
        return &mut self.cells;
    }
}
