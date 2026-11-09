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
}
