use macroquad::prelude::*;
use crate::consts::*;
use crate::paddle::Paddle;
use crate::ball::Ball;
use crate::effects::*;

pub struct Game {
    left_paddle: Paddle,
    right_paddle: Paddle,
    ball: Ball,
    left_score: u32,
    right_score: u32,
    phase: f32,
    particles: Vec<Particle>,
    screen_shake: f32,
    shake_offset: Vec2,
}

impl Game {
    pub fn new(two_players: bool) -> Self {
        let mut game = Self {
            left_paddle: Paddle::new(PADDLE_MARGIN + PADDLE_WIDTH / 2.0, false),
            right_paddle: Paddle::new(SCREEN_WIDTH - PADDLE_MARGIN - PADDLE_WIDTH / 2.0, !two_players),
            ball: Ball::new(),
            left_score: 0,
            right_score: 0,
            phase: 0.0,
            particles: Vec::new(),
            screen_shake: 0.0,
            shake_offset: Vec2::ZERO,
        };
        game.ball.reset();
        game
    }

    pub fn reset_ball(&mut self) {
        self.ball.reset();
    }

    pub fn update(&mut self, dt: f32) -> GameResult {
        self.phase += dt * 50.0;
        if self.phase >= 360.0 {
            self.phase -= 360.0;
        }

        if self.screen_shake > 0.0 {
            self.screen_shake -= dt * 5.0;
            self.shake_offset = Vec2::new(
                (macroquad::rand::gen_range(-10.0, 10.0)) * self.screen_shake,
                (macroquad::rand::gen_range(-10.0, 10.0)) * self.screen_shake,
            );
        } else {
            self.shake_offset = Vec2::ZERO;
        }

        let left_keys = (is_key_down(KeyCode::W), is_key_down(KeyCode::S));
        let right_keys = (is_key_down(KeyCode::Up), is_key_down(KeyCode::Down));

        let ball_pos_for_ai = if self.right_paddle.is_ai {
            Some(self.ball.position)
        } else {
            None
        };

        self.left_paddle.update(dt, None, left_keys);
        self.right_paddle.update(dt, ball_pos_for_ai, right_keys);

        if let Some(collision) = self.ball.update(dt, &self.left_paddle, &self.right_paddle) {
            self.screen_shake = 0.3;
            let explosion = create_particle_explosion(collision.position, collision.hue, 10);
            self.particles.extend(explosion);
        }

        self.particles.retain_mut(|p| {
            p.update(dt);
            p.is_alive()
        });

        if let Some(scored_right) = self.ball.scored() {
            if scored_right {
                self.right_score += 1;
                if self.right_score >= WIN_SCORE {
                    return GameResult::RightWins;
                }
            } else {
                self.left_score += 1;
                if self.left_score >= WIN_SCORE {
                    return GameResult::LeftWins;
                }
            }
            self.reset_ball();
        }

        GameResult::Continue
    }

    pub fn draw(&self) {
        set_default_camera();
        
        if self.shake_offset.length() > 0.0 {
            set_camera(&Camera2D {
                target: self.shake_offset,
                ..Default::default()
            });
        }

        let bg_color1 = get_rainbow_color(self.phase);
        let bg_color2 = get_rainbow_color((self.phase + 180.0) % 360.0);

        for y in 0..SCREEN_HEIGHT as i32 {
            let t = y as f32 / SCREEN_HEIGHT;
            let color = Color::new(
                bg_color1.r + (bg_color2.r - bg_color1.r) * t,
                bg_color1.g + (bg_color2.g - bg_color1.g) * t,
                bg_color1.b + (bg_color2.b - bg_color1.b) * t,
                0.05,
            );
            draw_line(0.0, y as f32, SCREEN_WIDTH, y as f32, 2.0, color);
        }

        let center_line_color = get_rainbow_color((self.phase + 90.0) % 360.0);
        for i in 0..20 {
            let y = (i as f32 * 30.0 + self.phase * 0.5) % SCREEN_HEIGHT;
            draw_rectangle(
                SCREEN_WIDTH / 2.0 - 2.0,
                y,
                4.0,
                15.0,
                Color::new(center_line_color.r, center_line_color.g, center_line_color.b, 0.6),
            );
        }

        self.left_paddle.draw(self.phase);
        self.right_paddle.draw(self.phase);
        self.ball.draw(self.phase);

        for particle in &self.particles {
            particle.draw(self.phase);
        }

        let border_color1 = get_rainbow_color(self.phase);
        let border_color2 = get_rainbow_color((self.phase + 60.0) % 360.0);
        let border_color3 = get_rainbow_color((self.phase + 120.0) % 360.0);
        let border_color4 = get_rainbow_color((self.phase + 180.0) % 360.0);

        draw_rectangle(0.0, 0.0, SCREEN_WIDTH, 5.0, border_color1);
        draw_rectangle(0.0, SCREEN_HEIGHT - 5.0, SCREEN_WIDTH, 5.0, border_color2);
        draw_rectangle(0.0, 0.0, 5.0, SCREEN_HEIGHT, border_color3);
        draw_rectangle(SCREEN_WIDTH - 5.0, 0.0, 5.0, SCREEN_HEIGHT, border_color4);

        let score_size = 60.0;
        let left_score_text = format!("{}", self.left_score);
        let right_score_text = format!("{}", self.right_score);

        let left_score_hue = (self.phase + 30.0) % 360.0;
        let right_score_hue = (self.phase + 210.0) % 360.0;

        draw_text_ex(
            &left_score_text,
            SCREEN_WIDTH / 4.0 - measure_text(&left_score_text, None, score_size as u16, 1.0).width / 2.0,
            50.0,
            TextParams {
                font: None,
                font_size: score_size as u16,
                color: get_rainbow_color(left_score_hue),
                ..Default::default()
            },
        );

        draw_text_ex(
            &right_score_text,
            SCREEN_WIDTH * 3.0 / 4.0 - measure_text(&right_score_text, None, score_size as u16, 1.0).width / 2.0,
            50.0,
            TextParams {
                font: None,
                font_size: score_size as u16,
                color: get_rainbow_color(right_score_hue),
                ..Default::default()
            },
        );

        set_default_camera();
    }

    pub fn draw_win_screen(&self, left_won: bool) {
        let bg_color1 = get_rainbow_color(self.phase);
        let bg_color2 = get_rainbow_color((self.phase + 120.0) % 360.0);

        for y in 0..SCREEN_HEIGHT as i32 {
            let t = y as f32 / SCREEN_HEIGHT;
            let color = Color::new(
                bg_color1.r + (bg_color2.r - bg_color1.r) * t,
                bg_color1.g + (bg_color2.g - bg_color1.g) * t,
                bg_color1.b + (bg_color2.b - bg_color1.b) * t,
                0.3,
            );
            draw_line(0.0, y as f32, SCREEN_WIDTH, y as f32, 2.0, color);
        }

        let win_text = if left_won { "LEFT PLAYER WINS!" } else { "RIGHT PLAYER WINS!" };
        let win_size = 50.0;
        let win_hue = self.phase;

        for offset in 0..8 {
            let glow_size = (offset as f32) * 3.0;
            let alpha = 0.4 / (offset as f32 + 1.0);
            let glow_color = get_rainbow_color((win_hue + offset as f32 * 30.0) % 360.0);
            draw_text_ex(
                win_text,
                SCREEN_WIDTH / 2.0 - measure_text(win_text, None, win_size as u16, 1.0).width / 2.0 + glow_size,
                SCREEN_HEIGHT / 2.0 - 50.0 + glow_size,
                TextParams {
                    font: None,
                    font_size: win_size as u16,
                    color: Color::new(glow_color.r, glow_color.g, glow_color.b, alpha),
                    ..Default::default()
                },
            );
        }

        draw_text_ex(
            win_text,
            SCREEN_WIDTH / 2.0 - measure_text(win_text, None, win_size as u16, 1.0).width / 2.0,
            SCREEN_HEIGHT / 2.0 - 50.0,
            TextParams {
                font: None,
                font_size: win_size as u16,
                color: get_rainbow_color(win_hue),
                ..Default::default()
            },
        );

        let press_text = "Press ENTER/SPACE to return to menu";
        let press_size = 25.0;
        draw_text_ex(
            press_text,
            SCREEN_WIDTH / 2.0 - measure_text(press_text, None, press_size as u16, 1.0).width / 2.0,
            SCREEN_HEIGHT / 2.0 + 50.0,
            TextParams {
                font: None,
                font_size: press_size as u16,
                color: Color::new(0.8, 0.8, 0.8, 0.9),
                ..Default::default()
            },
        );
    }

    pub fn draw_pause_screen(&self) {
        let overlay_alpha = 0.7;
        draw_rectangle(0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(0.0, 0.0, 0.0, overlay_alpha));

        let pause_text = "PAUSED";
        let pause_size = 70.0;
        let pause_hue = self.phase;

        for offset in 0..10 {
            let glow_size = (offset as f32) * 4.0;
            let alpha = 0.5 / (offset as f32 + 1.0);
            let glow_color = get_rainbow_color((pause_hue + offset as f32 * 25.0) % 360.0);
            draw_text_ex(
                pause_text,
                SCREEN_WIDTH / 2.0 - measure_text(pause_text, None, pause_size as u16, 1.0).width / 2.0 + glow_size,
                SCREEN_HEIGHT / 2.0 - 100.0 + glow_size,
                TextParams {
                    font: None,
                    font_size: pause_size as u16,
                    color: Color::new(glow_color.r, glow_color.g, glow_color.b, alpha),
                    ..Default::default()
                },
            );
        }

        draw_text_ex(
            pause_text,
            SCREEN_WIDTH / 2.0 - measure_text(pause_text, None, pause_size as u16, 1.0).width / 2.0,
            SCREEN_HEIGHT / 2.0 - 100.0,
            TextParams {
                font: None,
                font_size: pause_size as u16,
                color: get_rainbow_color(pause_hue),
                ..Default::default()
            },
        );

        let instruction_text = "Press P or ESC to resume";
        let instruction_size = 28.0;
        let instruction_hue = (self.phase + 180.0) % 360.0;
        
        draw_text_ex(
            instruction_text,
            SCREEN_WIDTH / 2.0 - measure_text(instruction_text, None, instruction_size as u16, 1.0).width / 2.0,
            SCREEN_HEIGHT / 2.0 + 20.0,
            TextParams {
                font: None,
                font_size: instruction_size as u16,
                color: get_rainbow_color(instruction_hue),
                ..Default::default()
            },
        );

        let score_info_size = 20.0;
        let left_score_text = format!("Left: {}", self.left_score);
        let right_score_text = format!("Right: {}", self.right_score);
        
        draw_text_ex(
            &left_score_text,
            50.0,
            SCREEN_HEIGHT / 2.0 + 80.0,
            TextParams {
                font: None,
                font_size: score_info_size as u16,
                color: get_rainbow_color((self.phase + 30.0) % 360.0),
                ..Default::default()
            },
        );

        draw_text_ex(
            &right_score_text,
            SCREEN_WIDTH - 150.0,
            SCREEN_HEIGHT / 2.0 + 80.0,
            TextParams {
                font: None,
                font_size: score_info_size as u16,
                color: get_rainbow_color((self.phase + 210.0) % 360.0),
                ..Default::default()
            },
        );

        let win_score_text = format!("First to {} wins", crate::consts::WIN_SCORE);
        draw_text_ex(
            &win_score_text,
            SCREEN_WIDTH / 2.0 - measure_text(&win_score_text, None, score_info_size as u16, 1.0).width / 2.0,
            SCREEN_HEIGHT / 2.0 + 120.0,
            TextParams {
                font: None,
                font_size: score_info_size as u16,
                color: Color::new(0.7, 0.7, 0.7, 0.8),
                ..Default::default()
            },
        );
    }
}

#[derive(PartialEq)]
pub enum GameResult {
    Continue,
    LeftWins,
    RightWins,
}

