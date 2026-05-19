use raylib::prelude::*;
pub struct Asteroid {
    pub position: Vector2,
    pub velocity: Vector2,
    pub hitbox_radius: f32,
    pub width: f32,
    pub height: f32,
    pub angle: f32,
    pub rotation_speed: f32,
}

impl Asteroid {
    pub fn new(position: Vector2, velocity: Vector2, hitbox_radius: f32, width: f32, height: f32, angle: f32, rotation_speed: f32) -> Self {
        Self {
            position,
            velocity,
            hitbox_radius,
            width,
            height,
            angle,
            rotation_speed
        }
    }

    pub fn update(&mut self, dt: f32, window_width: i32, window_height: i32) {
        self.position += self.velocity * dt;
        self.angle += self.rotation_speed * dt;

        if self.position.x < -self.width {
            self.position.x = window_width as f32;
        }
        if self.position.x > (window_width as f32) {
            self.position.x = -self.width;
        }
        if self.position.y < -self.height {
            self.position.y = window_height as f32;
        }
        if self.position.y > (window_height as f32) {
            self.position.y = -self.height;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        d.draw_texture_pro(
            texture,
            Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32),
            Rectangle::new(self.position.x, self.position.y, self.width, self.height),
            Vector2::new(self.width / 2.0, self.height / 2.0),
            self.angle.to_degrees(),
            Color::WHITE,
        );
    }
}