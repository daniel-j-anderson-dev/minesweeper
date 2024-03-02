use std::{fmt::Display, ops::Index};

use crate::cell::Cell;

/// A 2 dimensional board of `WIDTH` x `HEIGHT` [Cell]s in area.

#[derive(Clone, Copy)]
pub struct Board<const WIDTH: usize, const HEIGHT: usize> {
    cells: [[Cell; WIDTH]; HEIGHT],
}
impl<const W: usize, const H: usize> Index<(usize, usize)> for Board<W, H> {
    type Output = Cell;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        return self.cells().index(index.0).index(index.1);
    }
}
impl<const W: usize, const H: usize> Display for Board<W, H> {
    /// displays a board as a grid. rows delimited by new line, cells delimited by a space
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "   ")?;
        for column_index in 0..W {
            write!(f, "{} ", column_index)?;
        }
        write!(f, "\n   {}\n", String::from("-".repeat(W * 2)))?;
        for (row_index, row) in self.cells.iter().enumerate() {
            write!(f, "{} |", row_index)?;
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

    /// A helper associated function that returns a `W`x`H` 2d array of [Cell]s that each have a `is_mine_percentage` of being a mine
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
                let local_mine_count = Self::count_local_mines(&cells, (row, column));

                // set the local mine count for the given cell, based on the count accumulated
                cells[row][column].set_local_mines(local_mine_count);
            }
        }

        return cells;
    }

    /// count the number of [Cell]s that are mines surrounding a [Cell] at the specified indices
    fn count_local_mines(cells: &[[Cell; W]; H], index: (usize, usize)) -> usize {
        let mut local_mine_count = 0;

        // if the cell in question is not a mine
        if !cells[index.0][index.1].is_mine() {
            // list indices of all neighboring cells
            let neighbor_indices = [
                (index.0 as isize - 1, index.1 as isize - 1),
                (index.0 as isize - 1, index.1 as isize - 0),
                (index.0 as isize - 1, index.1 as isize + 1),
                (index.0 as isize - 0, index.1 as isize - 1),
                (index.0 as isize + 0, index.1 as isize + 1),
                (index.0 as isize + 1, index.1 as isize - 1),
                (index.0 as isize + 1, index.1 as isize + 0),
                (index.0 as isize + 1, index.1 as isize + 1),
            ];

            for (neighbor_row, neighbor_column) in neighbor_indices {
                // if the neighbor is in bounds
                if neighbor_row >= 0
                    && neighbor_column >= 0
                    && neighbor_row < H as isize
                    && neighbor_column < W as isize
                {
                    // Increment the local mine count
                    if cells[neighbor_row as usize][neighbor_column as usize].is_mine() {
                        local_mine_count += 1;
                    }
                }
            }
        }

        return local_mine_count;
    }

    /// This reflective function returns the first const parameter
    pub const fn width(&self) -> usize {
        return W;
    }
    /// This reflective function returns the second const parameter
    pub const fn height(&self) -> usize {
        return H;
    }

    /// This function returns a reference to a specified cell if the index is valid
    pub fn get_cell(&self, index: (usize, usize)) -> Option<&Cell> {
        return self
            .cells
            .get(index.0)
            .and_then(|row| row.get(index.1));
    }
    /// This function returns a mutable reference to a specified cell if the index is valid
    pub fn get_cell_mut(&mut self, index: (usize, usize)) -> Option<&mut Cell> {
        return self
            .cells
            .get_mut(index.0)
            .and_then(|row| row.get_mut(index.1));
    }
    /// This function returns a references to the [Board]'s`cells`
    pub fn cells(&self) -> &[[Cell; W]; H] {
        return &self.cells;
    }
    /// This function returns a mutable references to the [Board]'s`cells`
    pub fn cells_mut(&mut self) -> &mut [[Cell; W]; H] {
        return &mut self.cells;
    }

    /// Returns a copy of all of the cells revealed
    pub fn clone_revealed(&self) -> Self {
        let mut clone = Board::clone(&self);
        for row in clone.cells_mut() {
            for cell in row {
                cell.reveal();
            }
        }
        return clone;
    }
}
