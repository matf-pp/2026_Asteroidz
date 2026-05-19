mod controls;
mod projectile;
mod asteroids;
#[derive(PartialEq)]

enum ThrusterState {
    Off,
    Single,
    Triple,
}

use crate::controls::Controls;
use projectile::Projectile;
use asteroids::Asteroid;
use raylib::prelude::*;
use rand::{Rng, RngExt};

struct Player {
    position: Vector2,
    velocity: Vector2,
    box_size: Vector2,
    angle: f32,
    thruster_state: ThrusterState,
    thruster_timer: f32,
    health: u8,
}

impl Player {
    const SPEED: f32 = 1.00;
    const ROTATION_SPEED: f32 = 2.5;
    const ANIMATION_SPEED: f32 = 0.1;

    fn update(
        &mut self,
        rl: &RaylibHandle,
        window_width: i32,
        window_height: i32,
        controls: &Controls,
    ) {
        let dt = rl.get_frame_time();
        if rl.is_key_down(controls.right) {
            self.angle += Self::ROTATION_SPEED * dt;
        }
        if rl.is_key_down(controls.left) {
            self.angle -= Self::ROTATION_SPEED * dt;
        }
        if rl.is_key_down(controls.forward) {
            self.velocity.x += Self::SPEED * self.angle.sin();
            self.velocity.y -= Self::SPEED * self.angle.cos();

            if self.thruster_state == ThrusterState::Off {
                self.thruster_state = ThrusterState::Single;
            }
            self.thruster_timer += dt;
            if self.thruster_timer >= Self::ANIMATION_SPEED {
                self.thruster_timer = 0.0;
                self.thruster_state = match self.thruster_state {
                    ThrusterState::Single => ThrusterState::Triple,
                    _ => ThrusterState::Single,
                };
            }
        } else {
            self.thruster_state = ThrusterState::Off;
            self.thruster_timer = 0.0;
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

    fn _take_damage(&mut self) {
        self.health = self.health.saturating_sub(1);
    }

    fn _is_alive(&self) -> bool {
        return self.health > 0;
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

    let mut audio = RaylibAudio::init_audio_device();
    let mut volume = 1.0;
    let mut muted = false;
    audio.set_master_volume(volume);

    let mut player = Player {
        position: Vector2::new(100.0, 100.0),
        velocity: Vector2::new(0.0, 0.0),
        box_size: Vector2::new(32.0, 32.0),
        angle: 0.0,
        thruster_state: ThrusterState::Off,
        thruster_timer: 0.0,
        health: 3,
    };

    let texture_static = rl.load_texture(&thread, "assets/spaceship.png").unwrap();
    let texture_1thruster: Texture2D = rl.load_texture(&thread, "assets/1thruster.png").unwrap();
    let texture_3thruster: Texture2D = rl.load_texture(&thread, "assets/3thruster.png").unwrap();
    let heart_texture = rl.load_texture(&thread, "assets/heart.png").unwrap();
    let projectile_texture = rl.load_texture(&thread, "assets/projectile.png").unwrap();
    let asteroid_texture = rl.load_texture(&thread, "assets/asteroid.png").unwrap();

    let mut background_music =
        Music::load_music_stream(&thread, "assets/test_background.mp3").unwrap();
    background_music.looping = true;
    audio.play_music_stream(&mut background_music);
    let mut sfx_thruster = Music::load_music_stream(&thread, "assets/test_thruster.mp3").unwrap();
    sfx_thruster.looping = true;
    let mut is_thruster_sfx_playing = false; //ugly, will maybe find something better later
    let laser_pool = [
        Sound::load_sound("assets/test_laser.wav").unwrap(),
        Sound::load_sound("assets/test_laser.wav").unwrap(),
        Sound::load_sound("assets/test_laser.wav").unwrap(),
        Sound::load_sound("assets/test_laser.wav").unwrap(),
        Sound::load_sound("assets/test_laser.wav").unwrap(),
    ]; //doesn't seem to be any better solution to allow overlapping
    let mut current_laser = 0;

    let mut projectiles: Vec<Projectile> = Vec::new();

    let controls = Controls::new(None, None, None, None, None, None, None);

    let mut asteroids : Vec<Asteroid> = Vec::new();
    let mut rng = rand::rng();

    for i in 0..=2 {
        asteroids.push(Asteroid::new(
            Vector2::new(rng.random_range(0.0..window_width as f32), rng.random_range(0.0..window_height as f32)),
            Vector2::new(rng.random_range(-100.0..100.0), rng.random_range(-100.0..100.0)),
            60.0,
            64.0,
            64.0,
            rng.random_range(0.0..360.0),
            rng.random_range(-2.0..2.0),
        ));
    }

    while !rl.window_should_close() {
        audio.update_music_stream(&mut background_music);
        if rl.is_key_pressed(controls.mute){
            audio.set_master_volume(if !muted {0.0} else {volume});
            muted=!muted;
        }
        if rl.is_key_down(controls.volume_up){
            volume = (volume+0.01).min(1.0);
            if !muted {audio.set_master_volume(volume);}
        }
        if rl.is_key_down(controls.volume_down){
            volume=(volume-0.01).max(0.0);
            if !muted {audio.set_master_volume(volume);}
        }
        if player.thruster_state != ThrusterState::Off {
            audio.update_music_stream(&mut sfx_thruster);
            if !is_thruster_sfx_playing {
                audio.play_music_stream(&mut sfx_thruster);
                is_thruster_sfx_playing = true;
            }
        } else {
            if is_thruster_sfx_playing {
                audio.stop_music_stream(&mut sfx_thruster);
                is_thruster_sfx_playing = false;
            }
        }
        player.update(&rl, window_width, window_height, &controls);
        if rl.is_key_pressed(controls.shoot) {
            projectiles.push(Projectile::new(player.position, player.angle));

            audio.play_sound(&laser_pool[current_laser]);
            current_laser = (current_laser + 1) % laser_pool.len();
        }

        let dt = rl.get_frame_time();
        for proj in &mut projectiles {
            proj.update(dt);
        }

        for x in &mut asteroids {
            x.update(dt, window_width, window_height);
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        // d.draw_texture(&texture, player.position.x as i32, player.position.y as i32, Color::WHITE);

        let texture_current = match player.thruster_state {
            ThrusterState::Off => &texture_static,
            ThrusterState::Single => &texture_1thruster,
            ThrusterState::Triple => &texture_3thruster,
        };

        for proj in &projectiles {
            proj.draw(&mut d, &projectile_texture);
        }

        d.draw_texture_pro(
            &texture_current,
            Rectangle::new(
                0.0,
                0.0,
                texture_current.width as f32,
                texture_current.height as f32,
            ),
            Rectangle::new(
                player.position.x,
                player.position.y,
                player.box_size.x,
                player.box_size.y,
            ),
            Vector2::new(player.box_size.x / 2.0, player.box_size.y / 2.0),
            player.angle.to_degrees(),
            Color::WHITE,
        );

        for i in 0..player.health {
            // draw healthbar
            d.draw_texture(&heart_texture, 10 + (i as i32 * 25), 10, Color::WHITE);
        }

        for ast in &asteroids {
            ast.draw(&mut d, &asteroid_texture);
        }
    }
}
