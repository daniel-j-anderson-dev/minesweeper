use rand::Rng;
use std::fmt::Display;

fn main() {
    let minesweeper_board = Board::<5, 5>::initialize_random();
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
    pub fn random() -> Self {
        let mut rng = rand::thread_rng(); // random thread value
        return Self {
            is_mine: rng.gen_bool(0.5),
            local_mines: 0,
            is_revealed: false,
        };
    }
}

/// A 2 dimensional board of `WIDTH` x `HEIGHT` [Cell]s in area. 
struct Board<const WIDTH: usize, const HEIGHT: usize> {
    pub cells: [[Cell; WIDTH]; HEIGHT],
}
impl<const W: usize, const H: usize> Display for Board<W, H> {
    /// displays a board
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
    pub fn initialize_random() -> Self {
        let mut cells = [[Cell::CLEAR; W]; H];

        for row in cells.iter_mut() {
            for element in row.iter_mut() {
                *element = Cell::random();
            }
        }
        // place the array of cells in the board
        let mut board = Self { cells: cells };
        // determine the local mine count
        board.local_mine_count();

        return board;
    }
    /// determine the local mine count for each cell of the board and assigns it
    pub fn local_mine_count(&mut self) {
        let mut local_mine_count: usize = 0;

        // I didn't use an iterator bc I couldn't wrap my head around the logic for this given scenario...
        // It needed to be able to iterate across local cells within the board while also iterating across the entire board
        for row in 0..W {
            for col in 0..H {
                for i in (row as isize - 1)..=(row as isize + 1) {
                    for j in (col as isize - 1)..=(col as isize + 1) {
                        // Check if the neighboring cells are within bounds
                        if i >= 0
                            && j >= 0
                            && i < self.cells.len() as isize
                            && j < self.cells[i as usize].len() as isize
                        {
                            // Increment the local mine count
                            if self.cells[i as usize][j as usize].is_mine {
                                local_mine_count += 1;
                            }
                        }
                    }
                }

                // if the current cell has a mine it is subtracted from the local mine count
                if self.cells[row][col].is_mine {
                    local_mine_count -= 1;
                }

                // set the local mine count for the given cell, based on the count accumulated
                self.cells[row][col].local_mines = local_mine_count;
                // reset the local mine count for the next cell in the iteration
                local_mine_count = 0;
            }
        }
    }
}
