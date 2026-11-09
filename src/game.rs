use std::error::Error;

use raylib_utils::timer::Timer;
use sola_raylib::prelude::*;

use crate::{
    assets::{Assets, Textures},
    collision,
    entities::{Explosion, Input, Laser, Meteor, Player},
    settings::{
        BG_COLOR, FONT_SIZE, METEOR_TIMER_DURATION, STAR_COUNT, STAR_SCALE_MAX, STAR_SCALE_MIN,
        WINDOW_HEIGHT, WINDOW_WIDTH,
    },
};

struct Star {
    pos: Vector2,
    scale: f32,
}

pub struct Game<'aud> {
    assets: Assets<'aud>,
    player: Player,
    lasers: Vec<Laser>,
    meteors: Vec<Meteor>,
    meteor_timer: Timer,
    explosions: Vec<Explosion>,
    stars: Vec<Star>,
    score: u32,
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

        let stars = (0..STAR_COUNT)
            .map(|_| Star {
                pos: Vector2::new(
                    rand::random_range(0..=WINDOW_WIDTH) as f32,
                    rand::random_range(0..=WINDOW_HEIGHT) as f32,
                ),
                scale: rand::random_range(STAR_SCALE_MIN..=STAR_SCALE_MAX),
            })
            .collect();

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
            explosions: Vec::new(),
            stars,
            score: 0,
        })
    }

    pub fn run(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        while self.running && !rl.window_should_close() {
            if rl.is_key_pressed(KeyboardKey::KEY_F4) {
                rl.toggle_fullscreen();
            }

            self.update(rl);
            if !self.running {
                break;
            }

            self.draw(rl, thread);
        }

        self.assets.audio.music.stop_stream();
    }

    pub fn update(&mut self, rl: &RaylibHandle) {
        self.assets.audio.music.update_stream();
        let dt = rl.get_frame_time();
        for star in &mut self.stars {
            star.pos.y += 100.0 * dt;
            if star.pos.y > WINDOW_HEIGHT as f32 {
                star.pos.y = -10.0;
            }
        }

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

        let frame_count = self.assets.textures.explosion.len();
        for explosion in &mut self.explosions {
            explosion.update(dt, frame_count);
        }

        self.handle_collisions();
        self.discard();
    }

    fn handle_collisions(&mut self) {
        let frame_size = Textures::size_of(&self.assets.textures.explosion[0]);
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

                    self.explosions
                        .push(Explosion::new(meteor.sprite.center(), frame_size));
                    self.assets.audio.explosion.play();
                    self.score += 1;
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

        let screen_w = d.get_screen_width() as f32;
        let screen_h = d.get_screen_height() as f32;

        // Fit the fixed 1280x800 world into the current window, preserving
        // aspect ratio and centering it with letterbox bars.
        let scale = (screen_w / WINDOW_WIDTH as f32).min(screen_h / WINDOW_HEIGHT as f32);
        let offset = Vector2::new(
            (screen_w - WINDOW_WIDTH as f32 * scale) * 0.5,
            (screen_h - WINDOW_HEIGHT as f32 * scale) * 0.5,
        );

        d.clear_background(Color::BLACK);

        let camera = Camera2D {
            offset,
            target: Vector2::zero(),
            rotation: 0.0,
            zoom: scale,
        };

        d.draw_mode2D(camera, |mut d, _| {
            d.draw_rectangle(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, BG_COLOR);

            for star in &self.stars {
                d.draw_texture_ex(
                    &self.assets.textures.star,
                    star.pos,
                    0.0,
                    star.scale,
                    Color::WHITE,
                );
            }
            self.player
                .sprite
                .draw(&mut d, &self.assets.textures.player);
            for laser in &self.lasers {
                laser.sprite.draw(&mut d, &self.assets.textures.laser);
            }

            for meteor in &self.meteors {
                meteor.draw(&mut d, &self.assets.textures.meteor);
            }

            for explosion in &self.explosions {
                explosion.draw(&mut d, &self.assets.textures.explosion);
            }

            self.draw_score(&mut d);
        });
    }

    fn draw_score(&mut self, d: &mut RaylibDrawHandle<'_>) {
        let text = self.score.to_string();

        let size = self
            .assets
            .textures
            .font
            .measure_text(&text, FONT_SIZE as f32, 0.0);

        let pos = Vector2::new(WINDOW_WIDTH as f32 / 2.0 - size.x / 2.0, 100.0);

        d.draw_text_ex(
            &self.assets.textures.font,
            &text,
            pos,
            FONT_SIZE as f32,
            0.0,
            Color::WHITE,
        );
    }
}
