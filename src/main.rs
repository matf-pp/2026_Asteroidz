use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;


struct Player {
    position: Vector2,
    velocity: Vector2,
    box_size: Vector2,
    angle: f32,
}

impl Player {
    const SPEED: f32 = 1.00;
    const ROTATION_SPEED : f32 = 2.5;

    fn update(&mut self, rl: &RaylibHandle, window_width: i32, window_height: i32) {
        let dt = rl.get_frame_time();
        if rl.is_key_down(KEY_RIGHT) {
            self.angle += Self::ROTATION_SPEED * dt;
        } 
        if rl.is_key_down(KEY_LEFT) {
            self.angle -= Self::ROTATION_SPEED * dt;
        } 
        if rl.is_key_down(KEY_UP) {
            self.velocity.x += Self::SPEED * self.angle.sin();
            self.velocity.y -= Self::SPEED * self.angle.cos();
        }

        self.velocity *= 0.99;
        self.position += self.velocity * dt;

        if self.position.x < -self.box_size.x {
            self.position.x = window_width as f32;
        }
        if self.position.x > (window_width as f32) {
            self.position.x = -self.box_size.x;
        }
        if self.position.y < -self.box_size.y {
            self.position.y = window_height as f32;
        }
        if self.position.y > (window_height as f32) {
            self.position.y = -self.box_size.y;
        }
    }
}

fn main() {
    let window_width = 640;
    let window_height = 480;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Hello, World!")
        .vsync()
        .build();

    let mut player = Player {
        position: Vector2::new(100.0, 100.0),
        velocity: Vector2::new(0.0, 0.0),
        box_size: Vector2::new(32.0, 32.0),
        angle: 0.0,
    };

    let texture = rl.load_texture(&thread, "assets/spaceship.png").unwrap();

    while !rl.window_should_close() {
        player.update(&rl, window_width, window_height);
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        // d.draw_texture(&texture, player.position.x as i32, player.position.y as i32, Color::WHITE);
        d.draw_texture_pro(
            &texture,
            Rectangle::new(0.0, 0.0, texture.width as f32, texture.height as f32),
            Rectangle::new(player.position.x, player.position.y, player.box_size.x, player.box_size.y),
            Vector2::new(player.box_size.x / 2.0, player.box_size.y / 2.0),
            player.angle.to_degrees(),
            Color::WHITE
        )
    }
}