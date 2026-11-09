use sola_raylib::prelude::*;

use crate::{
    entities::Sprite,
    settings::{METEOR_SPEED_MAX, METEOR_SPEED_MIN, WINDOW_WIDTH},
};

pub struct Meteor {
    pub sprite: Sprite,
    rotation: f32,
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
            rotation: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.sprite.update(dt);
        self.rotation += 50.0 * dt;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>, texture: &Texture2D) {
        let source = Rectangle::new(0.0, 0.0, self.sprite.size.x, self.sprite.size.y);
        let target = Rectangle::new(
            self.sprite.pos.x,
            self.sprite.pos.y,
            self.sprite.size.x,
            self.sprite.size.y,
        );

        let origin = self.sprite.size / 2.0;

        d.draw_texture_pro(texture, source, target, origin, self.rotation, Color::WHITE);
    }
}
