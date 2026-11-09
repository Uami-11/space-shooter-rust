use sola_raylib::prelude::*;

use crate::{entities::Sprite, settings::LASER_SPEED};
#[derive(Debug)]
pub struct Laser {
    pub sprite: Sprite,
}

impl Laser {
    pub fn new(pos: Vector2, size: Vector2) -> Self {
        Self {
            sprite: Sprite::new(pos, size, Vector2::new(0.0, -1.0) * LASER_SPEED),
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        d.draw_rectangle_v(self.sprite.pos, self.sprite.size, Color::PURPLE);
    }
}
