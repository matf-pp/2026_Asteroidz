mod asteroids;
mod audio_manager;
mod controls;
mod player;
mod projectile;
mod textures;

#[derive(PartialEq)]
enum GameScreen {
    MainMenu,
    Gameplay,
    Paused, //not needed for now, but probably will implement later
}

use crate::player::ThrusterState;
use crate::{audio_manager::AudioManager, controls::Controls};
use asteroids::Asteroid;
use player::Player;
use projectile::Projectile;
use rand::{Rng, RngExt};
use raylib::prelude::*;
use textures::Textures;
fn main() {
    let window_width = 640;
    let window_height = 480;

    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("ASTEROIDZ")
        .vsync()
        .build();

    let mut active_screen = GameScreen::MainMenu;

    let mut player = Player::new(
        Vector2::new(100.0, 100.0),
        Vector2::new(0.0, 0.0),
        Vector2::new(32.0, 32.0),
        0.0,
        ThrusterState::Off,
    );

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

    let textures: Textures = Textures::init(
        texture_static,
        texture_1thruster,
        texture_3thruster,
        heart_texture,
        projectile_texture,
        asteroid_texture,
    );

    let mut projectiles: Vec<Projectile> = Vec::new();

    let mut asteroids: Vec<Asteroid> = Vec::new();

    let controls = Controls::new(None, None, None, None, None, None, None);

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
        poll_events(
            &mut rl,
            &mut active_screen,
            &mut player,
            &mut projectiles,
            &button_rect,
            &controls,
            &mut audio_manager,
        );

        update(
            &mut rl,
            &active_screen,
            window_width,
            window_height,
            &mut player,
            &mut asteroids,
            &mut projectiles,
            &controls,
            &mut audio_manager,
        );

        let mut d = rl.begin_drawing(&thread);

        draw(
            &mut d,
            &active_screen,
            window_width,
            window_height,
            &player,
            &asteroids,
            &projectiles,
            &textures,
            &button_rect,
        );
    }
}

fn draw(
    d: &mut RaylibDrawHandle,
    active_screen: &GameScreen,
    window_width: i32,
    window_height: i32,
    player: &Player,
    asteroids: &Vec<Asteroid>,
    projectiles: &Vec<Projectile>,
    texset: &Textures,
    button_rect: &Rectangle,
) {
    d.clear_background(Color::WHITE);
    match active_screen {
        GameScreen::MainMenu => {
            //TODO: replace hardcoded values to allow for rescaling
            d.draw_text(
                "BOTTOM TEXT",
                ((window_width as f32 / 2.0) - 115.0) as i32,
                ((window_height as f32 / 2.0) - 100.0) as i32,
                30,
                Color::BLACK,
            );
            d.draw_rectangle_rec(button_rect, Color::GREEN);
            d.draw_rectangle_lines_ex(button_rect, 2, Color::BLACK);
            d.draw_text(
                "START GAME",
                (button_rect.x + 35.0) as i32,
                (button_rect.y + 15.0) as i32,
                20,
                Color::BLACK,
            );
        }
        GameScreen::Gameplay => {
            let texture_current = match player.thruster_state {
                ThrusterState::Off => &texset.texture_static,
                ThrusterState::Single => &texset.texture_1thruster,
                ThrusterState::Triple => &texset.texture_3thruster,
            };

            for proj in projectiles {
                proj.draw(d, &texset.projectile_texture);
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
                d.draw_texture(
                    &texset.heart_texture,
                    10 + (i as i32 * 25),
                    10,
                    Color::WHITE,
                );
            }

            for ast in asteroids {
                ast.draw(d, &texset.asteroid_texture);
            }
        }
        GameScreen::Paused => {
            // TODO: implement...
        }
    }
}

fn poll_events(
    rl: &mut RaylibHandle,
    active_screen: &mut GameScreen,
    player: &mut Player,
    projectiles: &mut Vec<Projectile>,
    button_rect: &Rectangle,
    controls: &Controls,
    audio_manager: &mut AudioManager,
) {
    match active_screen {
        GameScreen::MainMenu => {
            let mouse_pos = rl.get_mouse_position();

            if button_rect.check_collision_point_rec(mouse_pos) {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                    *active_screen = GameScreen::Gameplay;
                }
            }
        }
        GameScreen::Gameplay => {
            if rl.is_key_pressed(controls.shoot) {
                projectiles.push(Projectile::new(player.position, player.angle));
                audio_manager.play_laser();
            }
        }
        GameScreen::Paused => {
            //future pause update logic, nothing so far
        }
    }
}

fn update(
    rl: &mut RaylibHandle,
    active_screen: &GameScreen,
    window_width: i32,
    window_height: i32,
    player: &mut Player,
    asteroids: &mut Vec<Asteroid>,
    projectiles: &mut Vec<Projectile>,
    controls: &Controls,
    audio_manager: &mut AudioManager,
) {
    audio_manager.update(&rl, &controls, &player.thruster_state);
    match active_screen {
        GameScreen::Gameplay => {
            player.update(&rl, window_width, window_height, &controls);

            projectiles.retain(|proj| {
                proj.position.x >= -10.0
                    && proj.position.x <= window_width as f32 + 10.0
                    && proj.position.y >= -10.0
                    && proj.position.y <= window_height as f32 + 10.0
            });

            asteroids.retain(|asrd| {
                asrd.position.x >= -10.0
                    && asrd.position.x <= window_width as f32 + 10.0
                    && asrd.position.y >= -10.0
                    && asrd.position.y <= window_height as f32 + 10.0
            });

            //println!("theres {} asteroids...", asteroids.len());

            let dt = rl.get_frame_time();
            for proj in projectiles {
                proj.update(dt);
            }

            for x in asteroids {
                x.update(dt, window_width, window_height);
                if player.is_alive() && Asteroid::check_collision_with_player(&player, x) {
                    player.take_damage();
                }
            }
        }
        _ => {}
    }
}
