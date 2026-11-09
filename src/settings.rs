use sola_raylib::color::Color;

pub const WINDOW_WIDTH: i32 = 2560 / 2;
pub const WINDOW_HEIGHT: i32 = 1600 / 2;
pub const WINDOW_TITLE: &str = "Space Shooter";
pub const BG_COLOR: Color = Color::new(25, 25, 25, 255);

pub const PLAYER_SPEED: f32 = 600.0;
pub const METEOR_SPEED_MIN: f32 = 300.0;
pub const METEOR_SPEED_MAX: f32 = 400.0;
pub const METEOR_TIMER_DURATION: f32 = 0.4;
pub const LASER_SPEED: f32 = 700.0;

pub const EXPLOSION_FPS: f32 = 20.0;

pub const FONT_SIZE: i32 = 120;

pub const STAR_COUNT: usize = 30;
pub const STAR_SCALE_MIN: f32 = 0.5;
pub const STAR_SCALE_MAX: f32 = 1.5;
