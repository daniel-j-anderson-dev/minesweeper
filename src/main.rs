use rand::Rng;
use std::io;
fn main() {
   
    let minesweeper_board = Board::initialize_random();
    minesweeper_board.display();
    

}

enum Cell {
    True,
    False,
    None,
}
struct Board {
    pub cells: [[bool; 5]; 5],
}
impl std::fmt::Display for Board {
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
impl Board {
    /// Initialize the minesweeper board with random true/false
    pub fn initialize_random() -> Self {
        let mut rng = rand::thread_rng(); // random thread value
        let mut new_board = [[true;5] ;5];
        
        for &row in new_board.iter() {
            for element in row.iter_mut() {
                *element = rng.gen_bool(0.5);
            }
        }


        todo!()
    }
}
