mod collision;
mod entities;
mod game;
mod settings;
use std::error::Error;

use crate::game::Game;
use crate::settings::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};

fn main() -> Result<(), Box<dyn Error>> {
    let (mut rl, thread) = sola_raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title(WINDOW_TITLE)
        .build();

    let mut game = Game::new();
    game.run(&mut rl, &thread);

    Ok(())
}
