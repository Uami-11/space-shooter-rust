use std::error::Error;

use raylib_utils::timer::Timer;
use sola_raylib::prelude::*;

use crate::{
    assets::{Assets, Textures},
    collision,
    entities::{Input, Laser, Meteor, Player},
    settings::{BG_COLOR, METEOR_TIMER_DURATION, WINDOW_HEIGHT, WINDOW_WIDTH},
};

pub struct Game<'aud> {
    assets: Assets<'aud>,
    player: Player,
    lasers: Vec<Laser>,
    meteors: Vec<Meteor>,
    meteor_timer: Timer,
    running: bool,
}

impl<'aud> Game<'aud> {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        audio: &'aud RaylibAudio,
    ) -> Result<Self, Box<dyn Error>> {
        let assets = Assets::load(rl, thread, audio)?;
        assets.audio.music.play_stream();

        let player_size = Textures::size_of(&assets.textures.player);

        Ok(Self {
            assets,
            running: true,
            player: Player::new(
                Vector2::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0),
                player_size,
            ),
            meteors: Vec::new(),
            meteor_timer: Timer::new(METEOR_TIMER_DURATION, true, true),
            lasers: Vec::new(),
        })
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
        self.assets.audio.music.update_stream();
        let dt = rl.get_frame_time();
        let input = Input::read(rl);
        if self.meteor_timer.tick(dt) {
            self.meteors
                .push(Meteor::new(Textures::size_of(&self.assets.textures.meteor)));
        }

        if input.shoot {
            self.spawn_laser();
        }

        for laser in &mut self.lasers {
            laser.sprite.update(dt);
        }

        self.player.update(&input, dt);

        for meteor in &mut self.meteors {
            meteor.update(dt);
        }

        self.handle_collisions();
        self.discard();
    }

    fn handle_collisions(&mut self) {
        for laser in &mut self.lasers {
            for meteor in &mut self.meteors {
                if laser.sprite.discard || meteor.sprite.discard {
                    continue;
                }

                if collision::circle_rect(
                    laser.sprite.rect(),
                    meteor.sprite.center(),
                    meteor.sprite.radius(),
                ) {
                    laser.sprite.discard = true;
                    meteor.sprite.discard = true;
                }
            }
        }

        let player_center = self.player.sprite.center();
        let player_radius = self.player.sprite.radius();
        let player_hit = self.meteors.iter().any(|meteor| {
            !meteor.sprite.discard
                && collision::circles(
                    player_center,
                    player_radius,
                    meteor.sprite.center(),
                    meteor.sprite.radius(),
                )
        });

        if player_hit {
            self.running = false;
        }
    }

    fn discard(&mut self) {
        self.meteors.retain(|meteor| !meteor.sprite.discard);
        self.lasers.retain(|laser| !laser.sprite.discard);
    }

    fn spawn_laser(&mut self) {
        self.assets.audio.laser.play();
        self.lasers.push(Laser::new(
            self.player.muzzle(),
            Textures::size_of(&self.assets.textures.laser),
        ));
    }

    pub fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(BG_COLOR);

        self.player
            .sprite
            .draw(&mut d, &self.assets.textures.player);
        for laser in &self.lasers {
            laser.sprite.draw(&mut d, &self.assets.textures.laser);
        }

        for meteor in &self.meteors {
            meteor.draw(&mut d, &self.assets.textures.meteor);
        }
    }
}
