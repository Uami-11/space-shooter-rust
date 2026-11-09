use sola_raylib::prelude::*;

use crate::settings::WINDOW_HEIGHT;

#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub pos: Vector2,
    pub velocity: Vector2,
    pub size: Vector2,
    pub discard: bool,
}

impl Sprite {
    pub fn new(pos: Vector2, size: Vector2, velocity: Vector2) -> Self {
        Self {
            pos,
            velocity,
            size,
            discard: false,
        }
    }

    pub fn advance(&mut self, dt: f32) {
        self.pos += self.velocity * dt;
    }

    pub fn update(&mut self, dt: f32) {
        self.advance(dt);
        self.discard = !(-300.0 < self.pos.y && self.pos.y < WINDOW_HEIGHT as f32 + 300.0);
    }

    pub fn center(&self) -> Vector2 {
        self.pos + (self.size / 2.0)
    }

    pub fn radius(&self) -> f32 {
        self.size.y / 2.0
    }

    pub fn rect(&self) -> Rectangle {
        Rectangle::new(self.pos.x, self.pos.y, self.size.x, self.size.y)
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        d.draw_texture_v(texture, self.pos, Color::WHITE);
    }
}
