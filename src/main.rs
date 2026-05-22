mod asteroids;
mod controls;
mod projectile;
mod audio_manager;
#[derive(PartialEq)]

pub enum ThrusterState {
    Off,
    Single,
    Triple,
}

#[derive(PartialEq)]
enum GameScreen {
    MainMenu,
    Gameplay,
    Paused, //not needed for now, but probably will implement later
}

use crate::{audio_manager::AudioManager, controls::Controls};
use asteroids::Asteroid;
use projectile::Projectile;
use rand::{Rng, RngExt};
use raylib::prelude::*;

struct Player {
    position: Vector2,
    velocity: Vector2,
    box_size: Vector2,
    angle: f32,
    thruster_state: ThrusterState,
    thruster_timer: f32,
    health: u8,
    invincible_timer: f32,
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

        if self.invincible_timer > 0.0 {
            self.invincible_timer -= dt;
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

    fn take_damage(&mut self) {
        if self.invincible_timer <= 0.0 {
            self.health = self.health.saturating_sub(1);
            self.invincible_timer = 2.0; // ship is invincible for 2s after hit
        }
    }

    fn is_alive(&self) -> bool {
        return self.health > 0;
    }
}

fn check_collision(player: &Player, asteroid: &Asteroid) -> bool {
    let dx = player.position.x - asteroid.position.x;
    let dy = player.position.y - asteroid.position.y;
    let distance = (dx * dx + dy * dy).sqrt();
    return distance < asteroid.hitbox_radius + player.box_size.x.max(player.box_size.y) / 2.0;
}

fn main() {
    let window_width = 640;
    let window_height = 480;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("ASTEROIDZ")
        .vsync()
        .build();

    let mut active_screen=GameScreen::MainMenu;

    let mut player = Player {
        position: Vector2::new(100.0, 100.0),
        velocity: Vector2::new(0.0, 0.0),
        box_size: Vector2::new(32.0, 32.0),
        angle: 0.0,
        thruster_state: ThrusterState::Off,
        thruster_timer: 0.0,
        health: 3,
        invincible_timer: 0.0,
    };

    let mut audio_manager = AudioManager::new(&thread);

    //TODO: replace hardcoded values to allow for scaling
    let button_rect = Rectangle::new(
        (window_width as f32 / 2.0) - 100.0,
        (window_height as f32 / 2.0) + 20.0, 
        200.0,
        50.0,
    );
    let texture_static = rl.load_texture(&thread, "assets/spaceship.png").unwrap();
    let texture_1thruster: Texture2D = rl.load_texture(&thread, "assets/1thruster.png").unwrap();
    let texture_3thruster: Texture2D = rl.load_texture(&thread, "assets/3thruster.png").unwrap();
    let heart_texture = rl.load_texture(&thread, "assets/heart.png").unwrap();
    let projectile_texture = rl.load_texture(&thread, "assets/projectile.png").unwrap();
    let asteroid_texture = rl.load_texture(&thread, "assets/asteroid.png").unwrap();

    let mut projectiles: Vec<Projectile> = Vec::new();

    let controls = Controls::new(None, None, None, None, None, None, None);

    let mut asteroids: Vec<Asteroid> = Vec::new();
    let mut rng = rand::rng();

    for _ in 0..=2 {
        asteroids.push(Asteroid::new(
            Vector2::new(
                rng.random_range(0.0..window_width as f32),
                rng.random_range(0.0..window_height as f32),
            ),
            Vector2::new(
                rng.random_range(-100.0..100.0),
                rng.random_range(-100.0..100.0),
            ),
            60.0,
            64.0,
            64.0,
            rng.random_range(0.0..360.0),
            rng.random_range(-2.0..2.0),
        ));
    }

    while !rl.window_should_close() {
        //UPDATE
        audio_manager.update(&rl, &controls, &player.thruster_state);

        match active_screen {
            GameScreen::MainMenu=>{
                let mouse_pos = rl.get_mouse_position();

                if button_rect.check_collision_point_rec(mouse_pos) {
                    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                        active_screen = GameScreen::Gameplay;
                    }
                }
            }
            GameScreen::Gameplay=>{
                player.update(&rl, window_width, window_height, &controls);
                if rl.is_key_pressed(controls.shoot) {
                    projectiles.push(Projectile::new(player.position, player.angle));
                    audio_manager.play_laser();
                }

                let dt = rl.get_frame_time();
                for proj in &mut projectiles {
                    proj.update(dt);
                }

                for x in &mut asteroids {
                    x.update(dt, window_width, window_height);
                    if player.is_alive() && check_collision(&player, x) {
                        player.take_damage();
                    }
                }
            }
            GameScreen::Paused=>{
                //future pause update logic, nothing so far
            }
        }

        //DRAW
        
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        // d.draw_texture(&texture, player.position.x as i32, player.position.y as i32, Color::WHITE);

        match active_screen{
            GameScreen::MainMenu=>{
                //TODO: replace hardcoded values to allow for rescaling
                d.draw_text("BOTTOM TEXT", ((window_width as f32 / 2.0) - 115.0) as i32,((window_height as f32 / 2.0) - 100.0) as i32, 30, Color::BLACK);
                d.draw_rectangle_rec(&button_rect, Color::GREEN);
                d.draw_rectangle_lines_ex(&button_rect, 2, Color::BLACK);
                d.draw_text("START GAME", (button_rect.x + 35.0) as i32, (button_rect.y + 15.0) as i32, 20, Color::BLACK);
            }
            GameScreen::Gameplay=>{
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
            GameScreen::Paused => {
                //eventual pause drawing
            }
        }

        
    }
}
