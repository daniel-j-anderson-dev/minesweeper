use rand::Rng;
use std::io;
fn main() {
   
    let minesweeper_board = initialize_random_board();
    display_board(&minesweeper_board);

}

struct Board {
    positions: [[bool; 5]; 5],
}

// outputs the minesweeper board
fn display_board(latest_board: &Board) {
    for &row in latest_board.positions.iter() {
        for &element in row.iter() {
            print!("{} ", element)
        }
        println!();
    }
}

// Initialize the minesweeper board with random true/false
fn initialize_random_board() -> Board {
    let mut rng = rand::thread_rng(); // random thread value
    let mut new_board = [[true;5] ;5];
    
    for &row in new_board.iter() {
        for element in row.iter_mut() {
            element = rng.gen_bool(0.5);
        }
    }
    let new_board = Board { positions: [[rng.gen_bool(0.5);5] ;5] }; // create board at random
    return new_board;
}
