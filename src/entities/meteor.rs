use sola_raylib::prelude::*;

use crate::{
    entities::Sprite,
    settings::{METEOR_SPEED_MAX, METEOR_SPEED_MIN, WINDOW_WIDTH},
};

pub struct Meteor {
    pub sprite: Sprite,
}

impl Meteor {
    pub fn new(size: Vector2) -> Self {
        let pos = Vector2::new(
            rand::random_range(0..=WINDOW_WIDTH) as f32,
            rand::random_range(-150..=-50) as f32,
        );

        let speed = rand::random_range(METEOR_SPEED_MIN..=METEOR_SPEED_MAX) as f32;
        let direction = Vector2::new(rand::random_range(-0.5..=0.5), 1.0);
        Self {
            sprite: Sprite::new(pos, size, direction * speed),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.sprite.update(dt);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        d.draw_rectangle_v(self.sprite.pos, self.sprite.size, Color::RED);
    }
}
