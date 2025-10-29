use macroquad::prelude::*;

pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub lifetime: f32,
    pub hue: f32,
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2, hue: f32) -> Self {
        Self {
            position,
            velocity,
            lifetime: 1.0,
            hue,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.lifetime -= dt;
        self.velocity *= 0.98;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    pub fn draw(&self, phase: f32) {
        let alpha = self.lifetime;
        let color_hue = (self.hue + phase) % 360.0;
        let color = hsv_to_rgb(color_hue / 360.0, 1.0, 1.0);
        draw_circle(
            self.position.x,
            self.position.y,
            3.0 * self.lifetime,
            Color::new(color.0, color.1, color.2, alpha),
        );
    }
}

pub struct TrailPoint {
    pub position: Vec2,
    pub time: f32,
    pub hue: f32,
}

pub struct Trail {
    pub points: Vec<TrailPoint>,
}

impl Trail {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn add_point(&mut self, position: Vec2, hue: f32) {
        self.points.push(TrailPoint {
            position,
            time: 1.0,
            hue,
        });
        if self.points.len() > crate::consts::TRAIL_LENGTH {
            self.points.remove(0);
        }
    }

    pub fn update(&mut self, dt: f32) {
        for point in &mut self.points {
            point.time -= dt * 2.0;
        }
        self.points.retain(|p| p.time > 0.0);
    }

    pub fn draw(&self, phase: f32) {
        for point in &self.points {
            let alpha = point.time * 0.6;
            let size = 5.0 * point.time;
            let color_hue = (point.hue + phase) % 360.0;
            let color = hsv_to_rgb(color_hue / 360.0, 1.0, 1.0);
            draw_circle(
                point.position.x,
                point.position.y,
                size,
                Color::new(color.0, color.1, color.2, alpha),
            );
        }
    }
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (r + m, g + m, b + m)
}

pub fn get_rainbow_color(hue: f32) -> Color {
    let rgb = hsv_to_rgb(hue / 360.0, 1.0, 1.0);
    Color::new(rgb.0, rgb.1, rgb.2, 1.0)
}

pub fn draw_glow(position: Vec2, size: f32, color: Color, intensity: f32) {
    for i in 0..5 {
        let alpha = intensity * (1.0 - i as f32 / 5.0) * 0.3;
        let scale = 1.0 + i as f32 * 0.3;
        draw_circle(
            position.x,
            position.y,
            size * scale,
            Color::new(color.r, color.g, color.b, alpha),
        );
    }
}

pub fn create_particle_explosion(position: Vec2, hue: f32, count: usize) -> Vec<Particle> {
    use crate::consts::PARTICLE_COUNT;
    let mut particles = Vec::new();
    let actual_count = count.min(PARTICLE_COUNT);
    
    for i in 0..actual_count {
        let angle = (i as f32 / actual_count as f32) * std::f32::consts::PI * 2.0;
        let speed = 100.0 + (i as f32 % 3.0) * 50.0;
        let velocity = Vec2::new(angle.cos() * speed, angle.sin() * speed);
        particles.push(Particle::new(position, velocity, hue));
    }
    
    particles
}

