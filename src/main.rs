// modules
mod board;
mod cell;
mod game;
mod input;

use crate::{
    game::Game,
    input::quit,
};

fn main() -> Result<(), std::io::Error> {
    let mut game = Game::new();

    loop {
        if game.execute_turn()?.is_game_over() {

            if quit()? {
                break;
            }

            game = Game::new();
        }
    }

    return Ok(());
}
