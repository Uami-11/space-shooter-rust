use sola_raylib::prelude::*;

use crate::settings::EXPLOSION_FPS;

pub struct Explosion {
    pub pos: Vector2,
    pub index: f32,
    pub discard: bool,
}

impl Explosion {
    pub fn new(center: Vector2, frame_size: Vector2) -> Self {
        Self {
            pos: Vector2::new(center.x - frame_size.x / 2.0, center.y - frame_size.y / 2.0),
            index: 0.0,
            discard: false,
        }
    }

    pub fn update(&mut self, dt: f32, frame_count: usize) {
        if frame_count == 0 {
            self.discard = true;
            return;
        }

        if self.index < (frame_count - 1) as f32 {
            self.index += EXPLOSION_FPS * dt;
        } else {
            self.discard = true;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle<'_>, frames: &[Texture2D]) {
        if frames.is_empty() {
            return;
        }

        let index = (self.index as usize).min(frames.len() - 1);

        d.draw_texture_v(&frames[index], self.pos, Color::WHITE);
    }
}
