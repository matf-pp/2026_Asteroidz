use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

struct Player {
    position: Vector2,
    box_size: Vector2,
    speed: f32,
}

impl Player {
    fn update(&mut self, rl: &RaylibHandle, window_width: i32, window_height: i32) {
        let dt = rl.get_frame_time();
        if rl.is_key_down(KEY_RIGHT) {
            self.position.x += self.speed * dt;
        } 
        if rl.is_key_down(KEY_LEFT) {
            self.position.x -= self.speed * dt;
        } 
        if rl.is_key_down(KEY_UP) {
            self.position.y -= self.speed * dt;
        }
        if rl.is_key_down(KEY_DOWN) {
            self.position.y += self.speed * dt;
        }

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
        box_size: Vector2::new(32.0, 32.0),
        speed: 200.0,
    };

    while !rl.window_should_close() {
        player.update(&rl, window_width, window_height);
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_rectangle_v(player.position, player.box_size, Color::RED);
    }
}