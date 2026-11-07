use sola_raylib::prelude::*;

use crate::{
    entities::{Input, Player},
    settings::{BG_COLOR, WINDOW_HEIGHT, WINDOW_WIDTH},
};

pub struct Game {
    player: Player,
    running: bool,
}

impl Game {
    pub fn new() -> Self {
        let player_size = Vector2::new(60.0, 60.0);
        Self {
            running: true,
            player: Player::new(
                Vector2::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0),
                player_size,
            ),
        }
    }

    pub fn run(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        while self.running && !rl.window_should_close() {
            self.update(rl);
            if !self.running {
                break;
            }

            self.draw(rl, thread);
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        let dt = rl.get_frame_time();
        let input = Input::read(rl);
        self.player.update(&input, dt);
    }
    pub fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(BG_COLOR);

        self.player.draw(&mut d);
    }
}
