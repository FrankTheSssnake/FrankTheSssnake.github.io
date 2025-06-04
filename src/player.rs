use macroquad::prelude::*;

pub struct Player {
    pub position: Vec2,
    pub speed: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: vec2(x, y),
            speed: 100.0,
        }
    }

    pub fn update(&mut self, delta: f32) {
        let mut direction = vec2(0.0, 0.0);
        if is_key_down(KeyCode::W) {
            direction.y -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            direction.y += 1.0;
        }
        if is_key_down(KeyCode::A) {
            direction.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            direction.x += 1.0;
        }

        self.position += direction.normalize_or_zero() * self.speed * delta;
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, 10.0, YELLOW);
    }
}

