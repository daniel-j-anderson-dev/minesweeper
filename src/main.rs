use rand::Rng;
use std::fmt::Display;
fn main() {
   
    let minesweeper_board = Board::<5, 5>::initialize_random();
    println!("{}", minesweeper_board);
    

}

#[derive(Clone, Copy)]
/// contains the information pertaining to a cell
struct Cell {
    is_mine: bool,
    /// number of mines surrounding this cell
    local_mines: usize,
}
impl Display for Cell {
    /// display as mine as "*". If empty, displays the number of mines in cells around it
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_mine {
            write!(f, "*")?;
        }else {
            write!(f, "{}", self.local_mines)?;
        }
        return Ok(());
    }
}
impl Cell {
    /// a clear cell used for a default value
    pub const CLEAR: Self = Self {
        is_mine: false,
        local_mines: 0,
    };
    /// used to generate a random cell
    pub fn random() -> Self {
        let mut rng = rand::thread_rng(); // random thread value
        return Self { is_mine: rng.gen_bool(0.5), local_mines: 0 };
    }
}
/// Board contains an 2D array of cells
struct Board<const WIDTH: usize, const HEIGHT: usize> {
    pub cells: [[Cell; WIDTH]; HEIGHT],
}
impl<const W: usize, const H: usize> Display for Board<W, H> {
    /// displays a board
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &row in self.cells.iter() {
            for &element in row.iter() {
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
        let mut cells = [[Cell::CLEAR; W] ; H];
        
        for row in cells.iter_mut() {
            for element in row.iter_mut() {
                *element = Cell::random();
            }
        }

        return Self {
            cells: cells,
        };
    }
}
