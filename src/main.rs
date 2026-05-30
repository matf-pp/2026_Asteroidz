mod asteroids;
mod audio_manager;
mod controls;
mod player;
mod projectile;
mod textures;

#[derive(PartialEq, Clone, Copy)]
enum GameScreen {
    MainMenu,
    Gameplay,

    ControlMenu,
    Paused,

    GameOver,
}

use std::collections::HashSet;

use crate::player::ThrusterState;
use crate::{audio_manager::AudioManager, controls::Controls, controls::Rebinding};
use asteroids::Asteroid;
use player::Player;
use projectile::Projectile;
use rand::{Rng, RngExt};
use raylib::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI};
use textures::Textures;

struct GameState {
    score: u32,
    last_asteroid_spawn_time: f64,
    asteroid_spawn_amount: u32,
}

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
        Vector2::new(
            (window_width as f32 - 32.0) / 2.0,
            (window_height as f32 - 32.0) / 2.0,
        ),
        Vector2::new(0.0, 0.0),
        Vector2::new(32.0, 32.0),
        0.0,
        ThrusterState::Off,
    );

    let mut current_game_state = GameState {
        score: 0,
        last_asteroid_spawn_time: rl.get_time(),
        asteroid_spawn_amount: 1,
    };

    let mut audio_manager = AudioManager::new(&thread);

    let button_rect_start = Rectangle::new(
        (window_width as f32 / 2.0) - 100.0,
        (window_height as f32 / 2.0) + 10.0,
        200.0,
        50.0,
    );
    let button_rect_controls = Rectangle::new(
        (window_width as f32 / 2.0) - 90.0,
        (window_height as f32 / 2.0) + 80.0,
        180.0,
        50.0,
    );
    let button_rect_back = Rectangle::new(
        (window_width as f32 / 2.0) - 110.0,
        (window_height as f32 / 2.0) + 80.0,
        220.0,
        50.0,
    );

    let mut blur_shader = rl
        .load_shader(&thread, None, Some("assets/blur.fs"))
        .unwrap();
    let res_loc = blur_shader.get_shader_location("renderResolution");
    blur_shader.set_shader_value(
        res_loc,
        Vector2::new(window_width as f32, window_height as f32),
    );
    let mut target = rl
        .load_render_texture(&thread, window_width as u32, window_height as u32)
        .unwrap();

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

    let mut controls = Controls::new(
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        window_width,
        window_height,
    );

    let mut rng = rand::rng();

    for _ in 0..=2 {
        let dimension: f32 = 2.0_f32.powf(6.0f32);
        asteroids.push(Asteroid::new(
            Vector2::new(
                rng.random_range(0.0..window_width as f32),
                rng.random_range(0.0..window_height as f32),
            ),
            Vector2::new(
                rng.random_range(-100.0..100.0),
                rng.random_range(-100.0..100.0),
            ),
            0.5 * dimension,
            dimension,
            dimension,
            rng.random_range(0.0..360.0),
            rng.random_range(-2.0..2.0),
        ));
    }

    let mut should_reset_game_state: bool = false;

    while !rl.window_should_close() {
        poll_events(
            &mut rl,
            &mut active_screen,
            &mut player,
            &mut projectiles,
            &button_rect_start,
            &button_rect_controls,
            &button_rect_back,
            &mut controls,
            &mut audio_manager,
            &mut should_reset_game_state,
        );

        update(
            &mut rl,
            &mut active_screen,
            window_width,
            window_height,
            &mut player,
            &mut asteroids,
            &mut projectiles,
            &controls,
            &mut audio_manager,
            &mut current_game_state,
            &mut should_reset_game_state,
        );
        let mut d = rl.begin_drawing(&thread);
        draw(
            &mut d,
            &active_screen,
            window_width,
            window_height,
            &player,
            &mut controls,
            &asteroids,
            &projectiles,
            &textures,
            &button_rect_start,
            &button_rect_controls,
            &button_rect_back,
            &mut target,
            &blur_shader,
            &thread,
            &mut current_game_state,
        );
    }
}

fn draw(
    d: &mut RaylibDrawHandle,
    active_screen: &GameScreen,
    window_width: i32,
    window_height: i32,
    player: &Player,
    controls: &mut Controls,
    asteroids: &Vec<Asteroid>,
    projectiles: &Vec<Projectile>,
    texset: &Textures,
    button_rect_start: &Rectangle,
    button_rect_controls: &Rectangle,
    button_rect_back: &Rectangle,
    target: &mut RenderTexture2D,
    blur_shader: &Shader,
    thread: &RaylibThread,
    current_game_state: &mut GameState,
) {
    d.clear_background(Color::DARKBLUE);
    match active_screen {
        GameScreen::MainMenu => {
            //TODO: replace hardcoded values to allow for rescaling
            let text = "ASTEROIDZ";
            let font_size = 40;
            let text_width = raylib::text::measure_text(text, font_size);
            d.draw_text(
                "ASTEROIDZ",
                ((window_width as f32 / 2.0) - (text_width as f32 / 2.0)) as i32,
                ((window_height as f32 / 2.0) - 100.0) as i32,
                font_size,
                Color::WHITE,
            );
            d.draw_rectangle_rec(button_rect_start, Color::GREEN);
            d.draw_rectangle_rec(button_rect_controls, Color::ORANGE);
            d.draw_rectangle_lines_ex(button_rect_start, 2, Color::BLACK);
            d.draw_rectangle_lines_ex(button_rect_controls, 2, Color::BLACK);
            d.draw_text(
                "START GAME",
                (button_rect_start.x + 35.0) as i32,
                (button_rect_start.y + 15.0) as i32,
                20,
                Color::BLACK,
            );
            d.draw_text(
                "CONTROLS",
                (button_rect_controls.x + 35.0) as i32,
                (button_rect_controls.y + 15.0) as i32,
                20,
                Color::BLACK,
            );
        }
        GameScreen::Gameplay => {
            {
                let mut texture_mode = d.begin_texture_mode(thread, target);
                texture_mode.clear_background(Color::DARKBLUE);

                for proj in projectiles {
                    proj.draw(&mut texture_mode, &texset.projectile_texture);
                }

                let texture_current = match player.thruster_state {
                    ThrusterState::Off => &texset.texture_static,
                    ThrusterState::Single => &texset.texture_1thruster,
                    ThrusterState::Triple => &texset.texture_3thruster,
                };

                if player.visible {
                    texture_mode.draw_texture_pro(
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
                }

                for ast in asteroids {
                    ast.draw(&mut texture_mode, &texset.asteroid_texture);
                }

                for i in 0..player.health {
                    texture_mode.draw_texture(
                        &texset.heart_texture,
                        10 + (i as i32 * 25),
                        10,
                        Color::WHITE,
                    );
                }
            }

            d.draw_texture_rec(
                target.texture(),
                Rectangle::new(
                    0.0,
                    0.0,
                    target.texture().width as f32,
                    -target.texture().height as f32,
                ),
                Vector2::new(0.0, 0.0),
                Color::WHITE,
            );

            let score_text = format!("SCORE: {}", current_game_state.score);
            d.draw_text(&score_text, window_height - 10, 10, 20, Color::WHITE);
        }
        GameScreen::ControlMenu => {
            Controls::draw_menu(d, controls);
        }
        GameScreen::Paused => {
            {
                let mut shader_mode = d.begin_shader_mode(blur_shader);
                shader_mode.draw_texture_rec(
                    target.texture(),
                    Rectangle::new(
                        0.0,
                        0.0,
                        target.texture().width as f32,
                        -target.texture().height as f32,
                    ),
                    Vector2::new(0.0, 0.0),
                    Color::WHITE,
                );
            }

            let text = "GAME PAUSED";
            let font_size = 40;
            let text_width = raylib::text::measure_text(text, font_size);
            d.draw_text(
                text,
                (window_width / 2) - (text_width / 2),
                (window_height / 2) - 20,
                font_size,
                Color::WHITE,
            );

            d.draw_rectangle_rec(button_rect_controls, Color::ORANGE);
            d.draw_rectangle_lines_ex(button_rect_controls, 2, Color::BLACK);
            d.draw_text(
                "CONTROLS",
                (button_rect_controls.x + 35.0) as i32,
                (button_rect_controls.y + 15.0) as i32,
                20,
                Color::BLACK,
            );
        }
        GameScreen::GameOver => {
            d.clear_background(Color::DARKBLUE);
            let text_go: &str = "GAME OVER";
            let mut font_size = 40;
            let mut text_width = raylib::text::measure_text(text_go, font_size);
            d.draw_text(
                text_go,
                ((window_width as f32 / 2.0) - (text_width as f32 / 2.0)) as i32,
                ((window_height as f32 / 2.0) - 100.0) as i32,
                font_size,
                Color::WHITE,
            );
            let text_score: String = format!("Score: {:?}", current_game_state.score);
            font_size = 30;
            text_width = raylib::text::measure_text(&text_score, font_size);
            d.draw_text(
                &text_score,
                ((window_width as f32 / 2.0) - (text_width as f32 / 2.0)) as i32,
                ((window_height as f32 / 2.0) - 10.0) as i32,
                font_size,
                Color::WHITE,
            );
            d.draw_rectangle_rounded_lines(button_rect_back, 0.4, 1, 1, Color::WHITE);
            d.draw_text(
                "Back to main menu",
                button_rect_controls.x as i32,
                (button_rect_controls.y + 15.0) as i32,
                20,
                Color::WHITE,
            );
        }
    }
}

fn poll_events(
    rl: &mut RaylibHandle,
    active_screen: &mut GameScreen,
    player: &mut Player,
    projectiles: &mut Vec<Projectile>,
    button_rect_start: &Rectangle,
    button_rec_controls: &Rectangle,
    button_rect_back: &Rectangle,
    controls: &mut Controls,
    audio_manager: &mut AudioManager,
    should_reset_game_state: &mut bool,
) {
    match active_screen {
        GameScreen::MainMenu => {
            let mouse_pos = rl.get_mouse_position();

            if button_rect_start.check_collision_point_rec(mouse_pos) {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                    *active_screen = GameScreen::Gameplay;
                }
            }

            if button_rec_controls.check_collision_point_rec(mouse_pos) {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                    controls.from_menu = GameScreen::MainMenu;
                    *active_screen = GameScreen::ControlMenu;
                }
            }
        }
        GameScreen::Gameplay => {
            if rl.is_key_pressed(controls.pause) {
                *active_screen = GameScreen::Paused;
            }
            if rl.is_key_pressed(controls.shoot) {
                if player.proj_delay <= 0.0 {
                    projectiles.push(Projectile::new(player.position, player.angle));
                    audio_manager.play_laser();
                    player.reset_proj_timer();
                }
            }
        }
        GameScreen::ControlMenu => {
            if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE)
                && controls.currently_rebinding == Rebinding::None
            {
                *active_screen = controls.from_menu;
            }

            Controls::poll_events(rl, controls);
        }
        GameScreen::Paused => {
            if rl.is_key_pressed(controls.pause) {
                *active_screen = GameScreen::Gameplay;
            }

            let mouse_pos = rl.get_mouse_position();
            if button_rec_controls.check_collision_point_rec(mouse_pos) {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                    controls.from_menu = GameScreen::Paused;
                    *active_screen = GameScreen::ControlMenu;
                }
            }
        }
        GameScreen::GameOver => {
            let mouse_pos = rl.get_mouse_position();
            if button_rect_back.check_collision_point_rec(mouse_pos) {
                if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                    *should_reset_game_state = true;
                    *active_screen = GameScreen::MainMenu;
                }
            }
        }
    }
}

fn update(
    rl: &mut RaylibHandle,
    active_screen: &mut GameScreen,
    window_width: i32,
    window_height: i32,
    player: &mut Player,
    asteroids: &mut Vec<Asteroid>,
    projectiles: &mut Vec<Projectile>,
    controls: &Controls,
    audio_manager: &mut AudioManager,
    current_game_state: &mut GameState,
    should_reset_game_state: &mut bool,
) {
    audio_manager.update(&rl, &controls, &player.thruster_state, &active_screen);

    match active_screen {
        GameScreen::Gameplay => {
            player.update(&rl, window_width, window_height, &controls);

            if !player.is_alive() {
                *active_screen = GameScreen::GameOver;
            }

            // projectiles.retain(|proj| {
            //     proj.position.x >= -10.0
            //         && proj.position.x <= window_width as f32 + 10.0
            //         && proj.position.y >= -10.0
            //         && proj.position.y <= window_height as f32 + 10.0
            // });

            // asteroids.retain(|asrd| {
            //     asrd.position.x >= -10.0
            //         && asrd.position.x <= window_width as f32 + 10.0
            //         && asrd.position.y >= -10.0
            //         && asrd.position.y <= window_height as f32 + 10.0
            // });

            //println!("theres {} asteroids...", asteroids.len());
            destroy_asteroids(asteroids, projectiles, current_game_state, audio_manager);

            let dt = rl.get_frame_time();
            let time: f64 = rl.get_time();

            for proj in projectiles {
                proj.update(dt);
            }

            for x in asteroids.iter_mut() {
                x.update(dt, window_width, window_height);
                if player.is_alive() && Asteroid::check_collision_with_player(&player, x) {
                    if player.invincible_timer <= 0.0 {
                        audio_manager.play_impact();
                    }
                    player.take_damage();
                }
            }

            spawn_new_asteroids(
                asteroids,
                window_width,
                window_height,
                current_game_state,
                time,
            );
        }
        GameScreen::MainMenu => {
            if *should_reset_game_state {
                *should_reset_game_state = false;
                reset_game_state(
                    player,
                    projectiles,
                    asteroids,
                    current_game_state,
                    window_width,
                    window_height,
                    rl,
                );
            }
        }
        _ => {}
    }
}

fn reset_game_state(
    player: &mut Player,
    projectiles: &mut Vec<Projectile>,
    asteroids: &mut Vec<Asteroid>,
    current_game_state: &mut GameState,
    window_width: i32,
    window_height: i32,
    rl: &mut RaylibHandle,
) {
    *player = Player::new(
        Vector2::new(
            (window_width as f32 - 32.0) / 2.0,
            (window_height as f32 - 32.0) / 2.0,
        ),
        Vector2::new(0.0, 0.0),
        Vector2::new(32.0, 32.0),
        0.0,
        ThrusterState::Off,
    );

    *current_game_state = GameState {
        score: 0,
        last_asteroid_spawn_time: rl.get_time(),
        asteroid_spawn_amount: 1,
    };

    projectiles.clear();
    asteroids.clear();

    let mut rng = rand::rng();

    for _ in 0..=2 {
        let dimension: f32 = 2.0_f32.powf(6.0f32);
        asteroids.push(Asteroid::new(
            Vector2::new(
                rng.random_range(0.0..window_width as f32),
                rng.random_range(0.0..window_height as f32),
            ),
            Vector2::new(
                rng.random_range(-100.0..100.0),
                rng.random_range(-100.0..100.0),
            ),
            0.5 * dimension,
            dimension,
            dimension,
            rng.random_range(0.0..360.0),
            rng.random_range(-2.0..2.0),
        ));
    }
}

fn destroy_asteroids(
    asteroids: &mut Vec<Asteroid>,
    projectiles: &mut Vec<Projectile>,
    current_game_state: &mut GameState,
    audio_manager: &mut AudioManager,
) {
    let mut proj_to_remove: HashSet<usize> = HashSet::new();
    let mut ast_to_remove: HashSet<usize> = HashSet::new();

    let mut rng = rand::rng();

    for (pi, proj) in projectiles.iter().enumerate() {
        for (ai, ast) in asteroids.iter().enumerate() {
            if Projectile::check_collision_with_asteroid(proj, ast) {
                current_game_state.score += 10;
                audio_manager.play_break();
                proj_to_remove.insert(pi);
                ast_to_remove.insert(ai);
            }
        }
    }

    let mut new_asteroids: Vec<Asteroid> = Vec::new();
    let mut new_projectiles: Vec<Projectile> = Vec::new();

    for i in 0..asteroids.len() {
        if ast_to_remove.contains(&i) {
            new_asteroids.extend(split_asteroid(&asteroids[i], &mut rng));
        } else {
            new_asteroids.push(asteroids[i].clone());
        }
    }

    for i in 0..projectiles.len() {
        if !proj_to_remove.contains(&i) {
            new_projectiles.push(projectiles[i].clone());
        }
    }

    *asteroids = new_asteroids;
    *projectiles = new_projectiles;
}

fn split_asteroid(ast: &Asteroid, rng: &mut impl Rng) -> Vec<Asteroid> {
    let angle_separation = rng.random_range(20.0_f32..60.0_f32).to_radians();
    let speed = (ast.velocity.x * ast.velocity.x + ast.velocity.y * ast.velocity.y).sqrt();
    let new_speed: f32 = speed * 1.2;
    let normalized: Vector2 = Vector2::new(ast.velocity.x / speed, ast.velocity.y / speed);

    let new_dimension = ast.width / 2.0;

    let rotate_vector = |v: Vector2, angle: f32| -> Vector2 {
        Vector2::new(
            v.x * angle.cos() - v.y * angle.sin(),
            v.x * angle.sin() + v.y * angle.cos(),
        )
    };

    if new_dimension < 32.0 {
        return Vec::new();
    }

    [-angle_separation, angle_separation]
        .iter()
        .map(|&angle| {
            Asteroid::new(
                ast.position,
                rotate_vector(normalized, angle) * new_speed,
                0.9 * new_dimension,
                new_dimension,
                new_dimension,
                rng.random_range(0.0..360.0),
                rng.random_range(-2.0..2.0),
            )
        })
        .collect()
}

fn spawn_new_asteroids(
    asteroids: &mut Vec<Asteroid>,
    window_width: i32,
    window_height: i32,
    current_game_state: &mut GameState,
    now: f64,
) {
    if now - current_game_state.last_asteroid_spawn_time >= 10.0 {
        let mut rng = rand::rng();
        let dimension: f32 = 2.0_f32.powf(6.0f32);

        for _ in 0..current_game_state.asteroid_spawn_amount {
            let possible_locations = vec![
                Vector2::new(-dimension, rng.random_range(0.0..window_height as f32)),
                Vector2::new(
                    window_width as f32,
                    rng.random_range(0.0..window_height as f32),
                ),
                Vector2::new(rng.random_range(0.0..window_width as f32), -dimension),
                Vector2::new(
                    rng.random_range(0.0..window_width as f32),
                    window_height as f32,
                ),
            ];

            let speed = rng.random_range(50.0..150.0);
            let possible_velocities = vec![
                {
                    let a = rng.random_range(-FRAC_PI_2..FRAC_PI_2);
                    Vector2::new(a.cos() * speed, a.sin() * speed)
                },
                {
                    let a = rng.random_range(FRAC_PI_2..3.0 * FRAC_PI_2);
                    Vector2::new(a.cos() * speed, a.sin() * speed)
                },
                {
                    let a = rng.random_range(0.0..PI);
                    Vector2::new(a.cos() * speed, a.sin() * speed)
                },
                {
                    let a = rng.random_range(-PI..0.0);
                    Vector2::new(a.cos() * speed, a.sin() * speed)
                },
            ];

            let index = rng.random_range(0..possible_locations.len());

            asteroids.push(Asteroid::new(
                possible_locations[index],
                possible_velocities[index],
                0.5 * dimension,
                dimension,
                dimension,
                rng.random_range(0.0..360.0),
                rng.random_range(-2.0..2.0),
            ));
        }

        current_game_state.last_asteroid_spawn_time = now;
        let new_spawn_amount =
            ((current_game_state.asteroid_spawn_amount as f32 * 1.5).ceil() as u32).min(5);
        current_game_state.asteroid_spawn_amount = new_spawn_amount;
    }
}
