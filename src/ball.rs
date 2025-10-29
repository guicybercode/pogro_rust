use macroquad::prelude::*;
use crate::consts::*;
use crate::effects::*;
use crate::paddle::Paddle;

pub struct Ball {
    pub position: Vec2,
    pub velocity: Vec2,
    pub speed: f32,
    pub hue: f32,
    pub trail: Trail,
}

impl Ball {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            velocity: Vec2::ZERO,
            speed: BALL_INITIAL_SPEED,
            hue: 120.0,
            trail: Trail::new(),
        }
    }

    pub fn reset(&mut self) {
        self.position = Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
        let angle = (macroquad::rand::gen_range(-45, 45) as f32).to_radians();
        let direction = if macroquad::rand::gen_range(0, 2) == 0 { -1.0 } else { 1.0 };
        self.velocity = Vec2::new(
            direction * angle.cos() * self.speed,
            angle.sin() * self.speed,
        );
        self.hue = (self.hue + 60.0) % 360.0;
    }

    pub fn update(&mut self, dt: f32, left_paddle: &Paddle, right_paddle: &Paddle) -> Option<CollisionResult> {
        self.position += self.velocity * dt;

        let collision = self.check_paddle_collision(left_paddle, right_paddle);
        
        if self.position.y <= BALL_SIZE / 2.0 || self.position.y >= SCREEN_HEIGHT - BALL_SIZE / 2.0 {
            self.velocity.y = -self.velocity.y;
            self.position.y = self.position.y
                .max(BALL_SIZE / 2.0)
                .min(SCREEN_HEIGHT - BALL_SIZE / 2.0);
        }

        self.trail.update(dt);
        self.trail.add_point(self.position, self.hue);

        if let Some(coll) = collision {
            Some(coll)
        } else if self.position.x < 0.0 || self.position.x > SCREEN_WIDTH {
            None
        } else {
            None
        }
    }

    fn check_paddle_collision(&mut self, left_paddle: &Paddle, right_paddle: &Paddle) -> Option<CollisionResult> {
        let ball_rect = self.get_rect();
        
        let left_rect = left_paddle.get_rect();
        let right_rect = right_paddle.get_rect();

        if ball_rect.overlaps(&left_rect) && self.velocity.x < 0.0 {
            self.handle_paddle_hit(left_paddle);
            return Some(CollisionResult {
                position: self.position,
                hue: self.hue,
            });
        }

        if ball_rect.overlaps(&right_rect) && self.velocity.x > 0.0 {
            self.handle_paddle_hit(right_paddle);
            return Some(CollisionResult {
                position: self.position,
                hue: self.hue,
            });
        }

        None
    }

    fn handle_paddle_hit(&mut self, paddle: &Paddle) {
        let paddle_center = paddle.get_center();
        let relative_y = (self.position.y - paddle_center.y) / (PADDLE_HEIGHT / 2.0);
        let bounce_angle = relative_y * 60.0_f32.to_radians();
        
        self.velocity = Vec2::new(
            -self.velocity.x.signum() * bounce_angle.cos() * self.speed,
            bounce_angle.sin() * self.speed,
        );

        self.position.x = if self.position.x < SCREEN_WIDTH / 2.0 {
            PADDLE_MARGIN + PADDLE_WIDTH + BALL_SIZE / 2.0
        } else {
            SCREEN_WIDTH - PADDLE_MARGIN - PADDLE_WIDTH - BALL_SIZE / 2.0
        };

        self.speed = (self.speed + BALL_SPEED_INCREASE).min(BALL_MAX_SPEED);
        self.hue = (self.hue + 30.0) % 360.0;
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(
            self.position.x - BALL_SIZE / 2.0,
            self.position.y - BALL_SIZE / 2.0,
            BALL_SIZE,
            BALL_SIZE,
        )
    }

    pub fn draw(&self, phase: f32) {
        self.trail.draw(phase);

        let color = get_rainbow_color((self.hue + phase) % 360.0);
        
        draw_glow(self.position, BALL_SIZE / 2.0, color, 1.5);

        draw_circle(self.position.x, self.position.y, BALL_SIZE / 2.0, color);

        let inner_color = Color::new(color.r * 1.3, color.g * 1.3, color.b * 1.3, 1.0);
        draw_circle(
            self.position.x,
            self.position.y,
            BALL_SIZE / 3.0,
            inner_color,
        );
    }

    pub fn scored(&self) -> Option<bool> {
        if self.position.x < 0.0 {
            Some(false)
        } else if self.position.x > SCREEN_WIDTH {
            Some(true)
        } else {
            None
        }
    }
}

pub struct CollisionResult {
    pub position: Vec2,
    pub hue: f32,
}

