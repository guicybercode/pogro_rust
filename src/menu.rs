use macroquad::prelude::*;
use crate::consts::*;
use crate::effects::*;

pub enum MenuChoice {
    None,
    OnePlayer,
    TwoPlayers,
}

pub struct Star {
    position: Vec2,
    size: f32,
    brightness: f32,
    speed: f32,
}

pub struct Menu {
    phase: f32,
    selected_option: usize,
    stars: Vec<Star>,
    floating_particles: Vec<(Vec2, Vec2, f32)>,
}

impl Menu {
    pub fn new() -> Self {
        let mut stars = Vec::new();
        for _ in 0..30 {
            stars.push(Star {
                position: Vec2::new(
                    macroquad::rand::gen_range(0.0, SCREEN_WIDTH),
                    macroquad::rand::gen_range(0.0, SCREEN_HEIGHT),
                ),
                size: macroquad::rand::gen_range(2.0, 5.0),
                brightness: macroquad::rand::gen_range(0.3, 1.0),
                speed: macroquad::rand::gen_range(20.0, 50.0),
            });
        }

        let mut floating_particles = Vec::new();
        for _ in 0..15 {
            floating_particles.push((
                Vec2::new(
                    macroquad::rand::gen_range(0.0, SCREEN_WIDTH),
                    macroquad::rand::gen_range(0.0, SCREEN_HEIGHT),
                ),
                Vec2::new(
                    macroquad::rand::gen_range(-30.0, 30.0),
                    macroquad::rand::gen_range(-30.0, 30.0),
                ),
                macroquad::rand::gen_range(0.0, 360.0),
            ));
        }

        Self {
            phase: 0.0,
            selected_option: 0,
            stars,
            floating_particles,
        }
    }

    pub fn update(&mut self, dt: f32) -> MenuChoice {
        self.phase += dt * 60.0;
        if self.phase >= 360.0 {
            self.phase -= 360.0;
        }

        for star in &mut self.stars {
            star.position.y += star.speed * dt;
            star.brightness = (self.phase * 0.1 + star.position.y * 0.01).sin() * 0.5 + 0.5;
            if star.position.y > SCREEN_HEIGHT + 10.0 {
                star.position.y = -10.0;
                star.position.x = macroquad::rand::gen_range(0.0, SCREEN_WIDTH);
            }
        }

        for (pos, vel, hue) in &mut self.floating_particles {
            *pos += *vel * dt;
            *hue += dt * 30.0;
            if *hue >= 360.0 {
                *hue -= 360.0;
            }
            if pos.x < 0.0 || pos.x > SCREEN_WIDTH {
                vel.x = -vel.x;
            }
            if pos.y < 0.0 || pos.y > SCREEN_HEIGHT {
                vel.y = -vel.y;
            }
            pos.x = pos.x.max(0.0).min(SCREEN_WIDTH);
            pos.y = pos.y.max(0.0).min(SCREEN_HEIGHT);
        }

        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            self.selected_option = if self.selected_option == 0 { 1 } else { 0 };
        }

        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            self.selected_option = if self.selected_option == 1 { 0 } else { 1 };
        }

        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            return match self.selected_option {
                0 => MenuChoice::OnePlayer,
                1 => MenuChoice::TwoPlayers,
                _ => MenuChoice::None,
            };
        }

        MenuChoice::None
    }

    pub fn draw(&self) {
        let bg_color1 = get_rainbow_color(self.phase);
        let bg_color2 = get_rainbow_color((self.phase + 120.0) % 360.0);
        let bg_color3 = get_rainbow_color((self.phase + 240.0) % 360.0);

        for star in &self.stars {
            let star_hue = (self.phase + star.position.x * 0.1) % 360.0;
            let star_color = get_rainbow_color(star_hue);
            draw_circle(
                star.position.x,
                star.position.y,
                star.size,
                Color::new(
                    star_color.r,
                    star_color.g,
                    star_color.b,
                    star.brightness * 0.8,
                ),
            );
            draw_circle(
                star.position.x,
                star.position.y,
                star.size * 0.5,
                Color::new(1.0, 1.0, 1.0, star.brightness),
            );
        }

        for (pos, _vel, hue) in &self.floating_particles {
            let particle_hue = (*hue + self.phase) % 360.0;
            let particle_color = get_rainbow_color(particle_hue);
            draw_circle(
                pos.x,
                pos.y,
                8.0,
                Color::new(particle_color.r, particle_color.g, particle_color.b, 0.6),
            );
            draw_circle(
                pos.x,
                pos.y,
                4.0,
                Color::new(particle_color.r * 1.5, particle_color.g * 1.5, particle_color.b * 1.5, 0.8),
            );
        }

        for y in 0..SCREEN_HEIGHT as i32 {
            let t = y as f32 / SCREEN_HEIGHT;
            let color = if t < 0.5 {
                Color::new(
                    bg_color1.r + (bg_color2.r - bg_color1.r) * t * 2.0,
                    bg_color1.g + (bg_color2.g - bg_color1.g) * t * 2.0,
                    bg_color1.b + (bg_color2.b - bg_color1.b) * t * 2.0,
                    0.15,
                )
            } else {
                Color::new(
                    bg_color2.r + (bg_color3.r - bg_color2.r) * (t - 0.5) * 2.0,
                    bg_color2.g + (bg_color3.g - bg_color2.g) * (t - 0.5) * 2.0,
                    bg_color2.b + (bg_color3.b - bg_color2.b) * (t - 0.5) * 2.0,
                    0.15,
                )
            };
            draw_line(0.0, y as f32, SCREEN_WIDTH, y as f32, 2.0, color);
        }

        let title_size = 80.0;
        let title_text = "PONG";
        
        for offset in 0..5 {
            let glow_size = (offset as f32) * 2.0;
            let alpha = 0.3 / (offset as f32 + 1.0);
            let glow_color = get_rainbow_color((self.phase + offset as f32 * 20.0) % 360.0);
            draw_text_ex(
                title_text,
                SCREEN_WIDTH / 2.0 - measure_text(title_text, None, title_size as u16, 1.0).width / 2.0 + glow_size,
                SCREEN_HEIGHT / 2.0 - 150.0 + glow_size,
                TextParams {
                    font: None,
                    font_size: title_size as u16,
                    color: Color::new(glow_color.r, glow_color.g, glow_color.b, alpha),
                    ..Default::default()
                },
            );
        }

        draw_text_ex(
            title_text,
            SCREEN_WIDTH / 2.0 - measure_text(title_text, None, title_size as u16, 1.0).width / 2.0,
            SCREEN_HEIGHT / 2.0 - 150.0,
            TextParams {
                font: None,
                font_size: title_size as u16,
                color: get_rainbow_color(self.phase),
                ..Default::default()
            },
        );

        let option_box_width = 300.0;
        let option_box_height = 70.0;
        let option_size = 40.0;
        let options = ["1 PLAYER", "2 PLAYERS"];

        for (i, option) in options.iter().enumerate() {
            let is_selected = i == self.selected_option;
            let y_pos = SCREEN_HEIGHT / 2.0 + (i as f32 * 80.0);
            let option_hue = (self.phase + i as f32 * 60.0) % 360.0;
            let box_x = SCREEN_WIDTH / 2.0 - option_box_width / 2.0;
            let box_y = y_pos - option_box_height / 2.0;

            let box_color = if is_selected {
                get_rainbow_color(option_hue)
            } else {
                Color::new(0.2, 0.2, 0.2, 0.4)
            };

            let glow_size = if is_selected { 10.0 } else { 0.0 };
            let pulse = if is_selected { (self.phase * 2.0).sin() * 0.3 + 0.7 } else { 1.0 };

            for offset in 0..5 {
                let glow_alpha = (0.3 / (offset as f32 + 1.0)) * pulse;
                let glow_color = get_rainbow_color((option_hue + offset as f32 * 20.0) % 360.0);
                draw_rectangle(
                    box_x - glow_size + offset as f32,
                    box_y - glow_size + offset as f32,
                    option_box_width + glow_size * 2.0 - offset as f32 * 2.0,
                    option_box_height + glow_size * 2.0 - offset as f32 * 2.0,
                    Color::new(glow_color.r, glow_color.g, glow_color.b, glow_alpha),
                );
            }

            if !is_selected {
                draw_rectangle(
                    box_x,
                    box_y,
                    option_box_width,
                    option_box_height,
                    box_color,
                );
            } else {
                draw_rectangle(
                    box_x,
                    box_y,
                    option_box_width,
                    option_box_height,
                    Color::new(box_color.r * 0.3, box_color.g * 0.3, box_color.b * 0.3, 0.6),
                );
            }

            let border_color = get_rainbow_color((option_hue + 90.0) % 360.0);
            draw_rectangle_lines(
                box_x,
                box_y,
                option_box_width,
                option_box_height,
                3.0,
                border_color,
            );

            if is_selected {
                let pulse = (self.phase * 2.0).sin() * 0.3 + 0.7;
                let selected_color = get_rainbow_color(option_hue);
                
                for offset in 0..3 {
                    let glow_size = (offset as f32) * 3.0;
                    let alpha = pulse * 0.6 / (offset as f32 + 1.0);
                    draw_text_ex(
                        option,
                        SCREEN_WIDTH / 2.0 - measure_text(option, None, option_size as u16, 1.0).width / 2.0 + glow_size,
                        y_pos + glow_size,
                        TextParams {
                            font: None,
                            font_size: option_size as u16,
                            color: Color::new(selected_color.r, selected_color.g, selected_color.b, alpha),
                            ..Default::default()
                        },
                    );
                }
            }

            let base_color = if is_selected {
                Color::new(1.0, 1.0, 1.0, 1.0)
            } else {
                Color::new(0.9, 0.9, 0.9, 0.9)
            };

            draw_text_ex(
                option,
                SCREEN_WIDTH / 2.0 - measure_text(option, None, option_size as u16, 1.0).width / 2.0,
                y_pos,
                TextParams {
                    font: None,
                    font_size: option_size as u16,
                    color: base_color,
                    ..Default::default()
                },
            );

            if is_selected {
                let arrow_x = SCREEN_WIDTH / 2.0 - measure_text(option, None, option_size as u16, 1.0).width / 2.0 - 40.0;
                let arrow_hue = (self.phase + 180.0) % 360.0;
                draw_text_ex(
                    ">",
                    arrow_x,
                    y_pos,
                    TextParams {
                        font: None,
                        font_size: option_size as u16,
                        color: get_rainbow_color(arrow_hue),
                        ..Default::default()
                    },
                );
            }
        }

        let version_size = 16.0;
        let version_text = "Version 1.0";
        let version_hue = (self.phase + 45.0) % 360.0;
        draw_text_ex(
            version_text,
            20.0,
            20.0,
            TextParams {
                font: None,
                font_size: version_size as u16,
                color: get_rainbow_color(version_hue),
                ..Default::default()
            },
        );

        let controls_size = 18.0;
        let controls_y_start = SCREEN_HEIGHT / 2.0 - 200.0;
        
        let controls_label_hue = self.phase;
        draw_text_ex(
            "CONTROLS:",
            SCREEN_WIDTH - 200.0,
            controls_y_start,
            TextParams {
                font: None,
                font_size: controls_size as u16,
                color: get_rainbow_color(controls_label_hue),
                ..Default::default()
            },
        );

        let left_hue = (self.phase + 30.0) % 360.0;
        draw_text_ex(
            "LEFT: W / S",
            SCREEN_WIDTH - 200.0,
            controls_y_start + 25.0,
            TextParams {
                font: None,
                font_size: controls_size as u16,
                color: get_rainbow_color(left_hue),
                ..Default::default()
            },
        );

        let right_text_x = SCREEN_WIDTH - 200.0;
        let right_text_y = controls_y_start + 50.0;
        let right_hue = (self.phase + 60.0) % 360.0;
        draw_text_ex(
            "RIGHT:",
            right_text_x,
            right_text_y,
            TextParams {
                font: None,
                font_size: controls_size as u16,
                color: get_rainbow_color(right_hue),
                ..Default::default()
            },
        );

        let arrow_start_x = right_text_x + measure_text("RIGHT: ", None, controls_size as u16, 1.0).width;
        let arrow_y = right_text_y;
        let arrow_size = 8.0;
        
        let up_arrow_hue = (self.phase + 90.0) % 360.0;
        let up_arrow_color = get_rainbow_color(up_arrow_hue);
        
        draw_line(arrow_start_x, arrow_y - arrow_size, arrow_start_x, arrow_y, 2.0, up_arrow_color);
        draw_line(arrow_start_x, arrow_y - arrow_size, arrow_start_x - arrow_size * 0.6, arrow_y - arrow_size * 0.3, 2.0, up_arrow_color);
        draw_line(arrow_start_x, arrow_y - arrow_size, arrow_start_x + arrow_size * 0.6, arrow_y - arrow_size * 0.3, 2.0, up_arrow_color);

        draw_text_ex(
            " / ",
            arrow_start_x + 15.0,
            arrow_y,
            TextParams {
                font: None,
                font_size: controls_size as u16,
                color: get_rainbow_color(right_hue),
                ..Default::default()
            },
        );

        let down_arrow_hue = (self.phase + 120.0) % 360.0;
        let down_arrow_color = get_rainbow_color(down_arrow_hue);
        let down_arrow_x = arrow_start_x + 35.0;
        
        draw_line(down_arrow_x, arrow_y + arrow_size, down_arrow_x, arrow_y, 2.0, down_arrow_color);
        draw_line(down_arrow_x, arrow_y + arrow_size, down_arrow_x - arrow_size * 0.6, arrow_y + arrow_size * 0.3, 2.0, down_arrow_color);
        draw_line(down_arrow_x, arrow_y + arrow_size, down_arrow_x + arrow_size * 0.6, arrow_y + arrow_size * 0.3, 2.0, down_arrow_color);

        let score_hue = (self.phase + 90.0) % 360.0;
        draw_text_ex(
            "SCORE TO 7 TO WIN",
            SCREEN_WIDTH - 200.0,
            controls_y_start + 75.0,
            TextParams {
                font: None,
                font_size: controls_size as u16,
                color: get_rainbow_color(score_hue),
                ..Default::default()
            },
        );

        let instruction_size = 20.0;
        let instruction = "Use ARROWS/W-S to navigate, ENTER/SPACE to select";
        draw_text_ex(
            instruction,
            SCREEN_WIDTH / 2.0 - measure_text(instruction, None, instruction_size as u16, 1.0).width / 2.0,
            SCREEN_HEIGHT - 100.0,
            TextParams {
                font: None,
                font_size: instruction_size as u16,
                color: Color::new(0.7, 0.7, 0.7, 0.8),
                ..Default::default()
            },
        );

        let credits_size = 18.0;
        let credits_text = "Made by cyberguicode";
        let credits_hue = (self.phase + 90.0) % 360.0;
        draw_text_ex(
            credits_text,
            SCREEN_WIDTH / 2.0 - measure_text(credits_text, None, credits_size as u16, 1.0).width / 2.0,
            SCREEN_HEIGHT - 50.0,
            TextParams {
                font: None,
                font_size: credits_size as u16,
                color: get_rainbow_color(credits_hue),
                ..Default::default()
            },
        );

        let copyright_size = 14.0;
        let copyright_text = "© 2025 - All rights reserved";
        draw_text_ex(
            copyright_text,
            SCREEN_WIDTH / 2.0 - measure_text(copyright_text, None, copyright_size as u16, 1.0).width / 2.0,
            SCREEN_HEIGHT - 25.0,
            TextParams {
                font: None,
                font_size: copyright_size as u16,
                color: Color::new(0.5, 0.5, 0.5, 0.6),
                ..Default::default()
            },
        );
    }
}

