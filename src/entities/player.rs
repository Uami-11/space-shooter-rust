use sola_raylib::prelude::*;

use crate::{
    entities::Sprite,
    settings::{PLAYER_SPEED, WINDOW_HEIGHT, WINDOW_WIDTH},
};

pub struct Input {
    pub direction: Vector2,
    pub shoot: bool,
}

impl Input {
    pub fn read(rl: &RaylibHandle) -> Self {
        let x = rl.is_key_down(KeyboardKey::KEY_RIGHT) as i32
            - rl.is_key_down(KeyboardKey::KEY_LEFT) as i32;

        let y = rl.is_key_down(KeyboardKey::KEY_DOWN) as i32
            - rl.is_key_down(KeyboardKey::KEY_UP) as i32;

        let direction = if x == 0 && y == 0 {
            Vector2::new(0.0, 0.0)
        } else {
            Vector2::new(x as f32, y as f32)
        };

        let shoot = rl.is_key_pressed(KeyboardKey::KEY_SPACE);

        Self { direction, shoot }
    }
}

pub struct Player {
    pub sprite: Sprite,
}

impl Player {
    pub fn new(pos: Vector2, size: Vector2) -> Self {
        Self {
            sprite: Sprite::new(pos, size, Vector2::new(0.0, 0.0)),
        }
    }
    pub fn update(&mut self, input: &Input, dt: f32) {
        self.sprite.velocity = input.direction * PLAYER_SPEED;
        self.sprite.advance(dt);

        self.sprite.pos.x = self
            .sprite
            .pos
            .x
            .clamp(0.0, WINDOW_WIDTH as f32 - self.sprite.size.x);

        self.sprite.pos.y = self
            .sprite
            .pos
            .y
            .clamp(0.0, WINDOW_HEIGHT as f32 - self.sprite.size.y);
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle<'_>) {
        d.draw_rectangle_v(self.sprite.pos, self.sprite.size, Color::WHITE)
    }

    pub fn muzzle(&mut self) -> Vector2 {
        Vector2::new(
            self.sprite.pos.x + (self.sprite.size.x / 2.0),
            self.sprite.pos.y - self.sprite.size.y,
        )
    }
}
