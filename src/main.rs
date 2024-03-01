use rand::Rng;
use std::fmt::Display;

fn main() {
    let minesweeper_board = Board::<5, 5>::random();
    println!("{}", minesweeper_board);

    for &row in minesweeper_board.cells.iter() {
        for &element in row.iter() {
            print!("{} ", element.local_mines);
        }
        println!();
    }
}

#[derive(Clone, Copy)]
/// contains the information pertaining to a cell
struct Cell {
    is_mine: bool,
    /// number of mines surrounding this cell
    local_mines: usize,
    /// determines if the user has picked to reveal contents of cell
    is_revealed: bool,
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
        } else {
            write!(f, " ")?;
        }
        return Ok(());
    }
}
impl Cell {
    /// a clear cell used for a default value
    pub const CLEAR: Self = Self {
        is_mine: false,
        local_mines: 0,
        is_revealed: false,
    };
    /// used to generate a random cell
    pub fn random(is_mine_percentage: f64) -> Self {
        let mut rng = rand::thread_rng(); // thread specific random number generator
        return Self {
            is_mine: rng.gen_bool(is_mine_percentage),
            local_mines: 0,
            is_revealed: false,
        };
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
    pub fn random() -> Self {
        // create cells that have a 50% chance of being a mine
        let cells = Board::random_cells(0.5);
        let cells = Board::initialize_local_mines(cells);
        
        return Board {
            cells: cells
        };
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
        let mut local_mine_count: usize = 0;

        for i in (row_index as isize - 1)..=(row_index as isize + 1) {
            for j in (column_index as isize - 1)..=(column_index as isize + 1) {
                // Check if the neighboring cells are within bounds
                if i >= 0
                    && j >= 0
                    && i < cells.len() as isize
                    && j < cells[i as usize].len() as isize
                {
                    // Increment the local mine count
                    if cells[i as usize][j as usize].is_mine {
                        local_mine_count += 1;
                    }
                }
            }
        }

        // if the current cell has a mine it is subtracted from the local mine count
        if cells[row_index][column_index].is_mine {
            local_mine_count -= 1;
        }

        return local_mine_count;
    }
}
