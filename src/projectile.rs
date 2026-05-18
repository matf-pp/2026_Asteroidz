use raylib::prelude::*;
pub struct Projectile {
    pub position: Vector2,
    velocity: Vector2,
    length: f32,
    width: f32,
    angle: f32,
}

impl Projectile {
    const SPEED: f32 = 400.0;

    pub fn new(position: Vector2, angle: f32) -> Self {
        Self {
            position,
            velocity: Vector2::new(angle.sin() * Self::SPEED, -angle.cos() * Self::SPEED),
            length: 32.0,
            width: 8.0,
            angle,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        d.draw_texture_pro(
            texture,
            Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32),
            Rectangle::new(
                self.position.x - 8.0,
                self.position.y,
                self.length,
                self.width,
            ),
            Vector2::new(self.width / 2.0, self.length / 2.0),
            self.angle.to_degrees() + 90.0,
            Color::WHITE,
        );
    }
}
