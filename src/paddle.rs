use macroquad::prelude::*;
use crate::consts::*;
use crate::effects::*;

pub struct Paddle {
    pub position: Vec2,
    pub velocity: f32,
    pub hue: f32,
    pub is_ai: bool,
    pub trail: Trail,
}

impl Paddle {
    pub fn new(x: f32, is_ai: bool) -> Self {
        Self {
            position: Vec2::new(x, SCREEN_HEIGHT / 2.0),
            velocity: 0.0,
            hue: if x < SCREEN_WIDTH / 2.0 { 0.0 } else { 180.0 },
            is_ai,
            trail: Trail::new(),
        }
    }

    pub fn update(&mut self, dt: f32, ball_position: Option<Vec2>, keys: (bool, bool)) {
        let target_velocity = if self.is_ai {
            self.ai_velocity(ball_position)
        } else {
            self.player_velocity(keys)
        };

        self.velocity += (target_velocity - self.velocity) * 15.0 * dt;
        self.position.y += self.velocity * dt;

        self.position.y = self.position.y
            .max(PADDLE_HEIGHT / 2.0)
            .min(SCREEN_HEIGHT - PADDLE_HEIGHT / 2.0);

        self.trail.update(dt);
        self.trail.add_point(Vec2::new(self.position.x, self.position.y), self.hue);
    }

    fn player_velocity(&self, keys: (bool, bool)) -> f32 {
        match keys {
            (true, false) => -PADDLE_SPEED,
            (false, true) => PADDLE_SPEED,
            _ => 0.0,
        }
    }

    fn ai_velocity(&self, ball_position: Option<Vec2>) -> f32 {
        if let Some(ball_pos) = ball_position {
            let target_y = ball_pos.y;
            let diff = target_y - self.position.y;
            let threshold = 20.0;

            if diff.abs() > threshold {
                diff.signum() * PADDLE_SPEED * 0.85
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(
            self.position.x - PADDLE_WIDTH / 2.0,
            self.position.y - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        )
    }

    pub fn draw(&self, phase: f32) {
        self.trail.draw(phase);

        let color = get_rainbow_color((self.hue + phase) % 360.0);
        
        draw_glow(
            self.position,
            PADDLE_HEIGHT / 2.0,
            color,
            1.0,
        );

        let rect = self.get_rect();
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);

        let inner_color = Color::new(color.r * 1.5, color.g * 1.5, color.b * 1.5, 1.0);
        draw_rectangle(
            rect.x + 3.0,
            rect.y + 3.0,
            rect.w - 6.0,
            rect.h - 6.0,
            inner_color,
        );
    }

    pub fn get_center(&self) -> Vec2 {
        self.position
    }
}

