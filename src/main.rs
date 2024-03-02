// modules
mod board;
mod cell;
mod game;
mod input;

use crate::{
    game::{Game, GameOver},
    input::quit,
};

fn main() -> Result<(), std::io::Error> {
    let mut game = Game::new();

    loop {
        if let Some(GameOver) = game.execute_turn()? {

            if quit()? {
                break;
            }

            game = Game::new();
        }
    }

    return Ok(());
}
