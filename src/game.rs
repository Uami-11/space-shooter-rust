use raylib_utils::timer::Timer;
use sola_raylib::prelude::*;

use crate::{
    entities::{Input, Meteor, Player},
    settings::{BG_COLOR, METEOR_TIMER_DURATION, WINDOW_HEIGHT, WINDOW_WIDTH},
};

pub struct Game {
    player: Player,
    meteors: Vec<Meteor>,
    meteor_timer: Timer,
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
            meteors: Vec::new(),
            meteor_timer: Timer::new(METEOR_TIMER_DURATION, true, true),
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
        if self.meteor_timer.tick(dt) {
            self.meteors.push(Meteor::new(Vector2::new(60.0, 60.0)));
        }

        self.player.update(&input, dt);

        for meteor in &mut self.meteors {
            meteor.update(dt);
        }

        self.discard();
    }

    pub fn discard(&mut self) {
        self.meteors.retain(|meteor| !meteor.sprite.discard);
    }

    pub fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(BG_COLOR);

        self.player.draw(&mut d);

        for meteor in &self.meteors {
            meteor.draw(&mut d);
        }
    }
}
