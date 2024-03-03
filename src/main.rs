// modules
mod board;
mod cell;
mod game;
mod input;

use game::Game;

use color_eyre::Report;
// use input::quit;
use macroquad::prelude::*;

#[macroquad::main("Minesweeper")]
async fn main() -> Result<(), Report> {
    let mut game = Game::new();

    loop {
        clear_background(SKYBLUE);

        game.draw();

        let game_state = game.update();

        if game_state.is_game_over() {
            game = Game::new();
        }

        next_frame().await;
    }

    return Ok(());
}
