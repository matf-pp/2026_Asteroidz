use crate::asteroids::Asteroid;
use raylib::prelude::*;

#[derive(Clone)]
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

    pub fn draw(&self, d: &mut impl RaylibDraw, texture: &Texture2D) {
        d.draw_texture_pro(
            texture,
            Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32),
            Rectangle::new(self.position.x, self.position.y, self.length, self.width),
            Vector2::new(self.length / 2.0, self.width / 2.0),
            self.angle.to_degrees() + 90.0,
            Color::WHITE,
        );
    }

    pub fn check_collision_with_asteroid(proj: &Projectile, ast: &Asteroid) -> bool {
        let tip = proj.position
            + Vector2::new(
                proj.angle.sin() * proj.length / 2.0,
                -proj.angle.cos() * proj.length / 2.0,
            );

        let dx = tip.x - ast.position.x;
        let dy = tip.y - ast.position.y;
        dx * dx + dy * dy < ast.hitbox_radius * ast.hitbox_radius
    }
}
