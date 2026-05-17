use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

struct Player {
    position: Vector2,
    boxSize: Vector2,
    speed: f32,
}

fn main() {
    let windowWidth = 640;
    let windowHeight = 480;

    let (mut rl, thread) = raylib::init()
        .size(windowWidth, windowHeight)
        .title("Hello, World!")
        .vsync()
        .build();

    let mut player = Player {
        position: Vector2::new(100.0, 100.0),
        boxSize: Vector2::new(10.0, 10.0),
        speed: 200.0,
    };

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        if rl.is_key_down(KEY_RIGHT) {
            player.position.x += player.speed * dt;
        } 
        if rl.is_key_down(KEY_LEFT) {
            player.position.x -= player.speed * dt;
        } 
        if rl.is_key_down(KEY_UP) {
            player.position.y -= player.speed * dt;
        }
        if rl.is_key_down(KEY_DOWN) {
            player.position.y += player.speed * dt;
        }

        if player.position.x < -player.boxSize.x {
            player.position.x = windowWidth as f32;
        }
        if player.position.x > (windowWidth as f32) + player.boxSize.x {
            player.position.x = 0.0;
        }
        if player.position.y < -player.boxSize.y {
            player.position.y = windowHeight as f32;
        }
        if player.position.y > (windowHeight as f32) + player.boxSize.y {
            player.position.y = 0.0;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_rectangle_v(player.position, player.boxSize, Color::RED);
    }
}