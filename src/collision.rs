use sola_raylib::prelude::*;

pub fn circles(a: Vector2, radius_a: f32, b: Vector2, radius_b: f32) -> bool {
    check_collision_circles(a, radius_a, b, radius_b)
}

pub fn circle_rect(rect: Rectangle, center: Vector2, radius: f32) -> bool {
    rect.check_collision_circle_rec(center, radius)
}
