use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;

use crate::GameScreen;

pub struct Controls {
    pub left: KeyboardKey,
    pub right: KeyboardKey,
    pub forward: KeyboardKey,
    pub shoot: KeyboardKey,
    pub mute: KeyboardKey,
    pub volume_up: KeyboardKey,
    pub volume_down: KeyboardKey,
    pub pause: KeyboardKey,

    currently_rebinding: Rebinding,
    display_rebind_err: bool,
    pub from_menu: GameScreen,

    button_rect_front: Rectangle,
    button_rect_left: Rectangle,
    button_rect_right: Rectangle,
    button_rect_shoot: Rectangle,

    window_width: i32,
    window_height: i32,
}

#[derive(PartialEq)]
enum Rebinding {
    None,
    Forward,
    Left,
    Right,
    Shoot,
}

impl Controls {
    pub fn new(
        left: Option<KeyboardKey>,
        right: Option<KeyboardKey>,
        forward: Option<KeyboardKey>,
        shoot: Option<KeyboardKey>,
        mute: Option<KeyboardKey>,
        volume_up: Option<KeyboardKey>,
        volume_down: Option<KeyboardKey>,
        pause: Option<KeyboardKey>,
        window_width: i32,
        window_height: i32,
    ) -> Self {
        Self {
            left: left.unwrap_or(KEY_LEFT),
            right: right.unwrap_or(KEY_RIGHT),
            forward: forward.unwrap_or(KEY_UP),
            shoot: shoot.unwrap_or(KEY_S),
            mute: mute.unwrap_or(KEY_M),
            volume_up: volume_up.unwrap_or(KEY_F3),
            volume_down: volume_down.unwrap_or(KEY_F2),
            pause: pause.unwrap_or(KEY_P),
            currently_rebinding: Rebinding::None,
            display_rebind_err: false,
            from_menu: GameScreen::MainMenu,

            button_rect_front: Rectangle::new(
                ((window_width as f32 / 2.0) + 150.0) as f32,
                (window_height as f32 / 2.0 - 180.0) as f32,
                125.0,
                50.0,
            ),
            button_rect_left: Rectangle::new(
                ((window_width as f32 / 2.0) + 150.0) as f32,
                (window_height as f32 / 2.0 - 100.0) as f32,
                125.0,
                50.0,
            ),
            button_rect_right: Rectangle::new(
                ((window_width as f32 / 2.0) + 150.0) as f32,
                (window_height as f32 / 2.0 - 20.0) as f32,
                125.0,
                50.0,
            ),
            button_rect_shoot: Rectangle::new(
                ((window_width as f32 / 2.0) + 150.0) as f32,
                (window_height as f32 / 2.0 + 60.0) as f32,
                125.0,
                50.0,
            ),

            window_width: window_width,
            window_height: window_height,
        }
    }
    fn key_to_string(key: KeyboardKey) -> String {
        format!("{:?}", key).replace("KEY_", "")
    }

    fn code_to_key(code: u32) -> KeyboardKey {
        // just why?? why does it return u32 but want i32... 💀
        // HACK: if anyone finds a better way, be my guest
        unsafe { std::mem::transmute(code as i32) }
    }
    pub fn poll_events(rl: &mut RaylibHandle, controls: &mut Controls) {
        // choose which action to rebind
        match controls.currently_rebinding {
            Rebinding::Forward | Rebinding::Left | Rebinding::Right | Rebinding::Shoot => {
                if let Some(pressed) = rl.get_key_pressed_number() {
                    let key = Controls::code_to_key(pressed);
                    if !controls.key_already_in_use(key, &controls.currently_rebinding) {
                        match controls.currently_rebinding {
                            Rebinding::Forward => controls.forward = key,
                            Rebinding::Left => controls.left = key,
                            Rebinding::Right => controls.right = key,
                            Rebinding::Shoot => controls.shoot = key,
                            _ => {}
                        }
                        controls.currently_rebinding = Rebinding::None;
                        controls.display_rebind_err = false;
                    } else {
                        controls.display_rebind_err = true;
                    }
                }
            }
            Rebinding::None => {
                let mouse_pos = rl.get_mouse_position();

                if controls
                    .button_rect_front
                    .check_collision_point_rec(mouse_pos)
                {
                    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                        controls.currently_rebinding = Rebinding::Forward;
                    }
                } else if controls
                    .button_rect_left
                    .check_collision_point_rec(mouse_pos)
                {
                    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                        controls.currently_rebinding = Rebinding::Left;
                    }
                } else if controls
                    .button_rect_right
                    .check_collision_point_rec(mouse_pos)
                {
                    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                        controls.currently_rebinding = Rebinding::Right;
                    }
                } else if controls
                    .button_rect_shoot
                    .check_collision_point_rec(mouse_pos)
                {
                    if rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                        controls.currently_rebinding = Rebinding::Shoot;
                    }
                }
            }
        }
    }

    fn key_already_in_use(&self, key: KeyboardKey, rebinding: &Rebinding) -> bool {
        match rebinding {
            Rebinding::Forward => self.left == key || self.right == key || self.shoot == key,
            Rebinding::Left => self.forward == key || self.right == key || self.shoot == key,
            Rebinding::Right => self.left == key || self.forward == key || self.shoot == key,
            Rebinding::Shoot => self.left == key || self.right == key || self.forward == key,
            _ => false,
        }
    }
    pub fn draw_menu(d: &mut RaylibDrawHandle, controls: &mut Controls) {
        let center_x = controls.window_width as f32 / 2.0;
        let center_y = controls.window_height as f32 / 2.0;

        let entries = [
            (
                "Move forward:",
                controls.button_rect_front,
                controls.forward,
                Rebinding::Forward,
                -180.0,
            ),
            (
                "Turn left:",
                controls.button_rect_left,
                controls.left,
                Rebinding::Left,
                -100.0,
            ),
            (
                "Turn right:",
                controls.button_rect_right,
                controls.right,
                Rebinding::Right,
                -20.0,
            ),
            (
                "Shoot:",
                controls.button_rect_shoot,
                controls.shoot,
                Rebinding::Shoot,
                60.0,
            ),
        ];

        for (label, rect, key, rebinding, y_offset) in entries {
            let y = center_y + y_offset;

            d.draw_text(label, (center_x - 200.0) as i32, y as i32, 25, Color::BLACK);

            d.draw_rectangle_lines_ex(
                rect,
                2,
                if controls.currently_rebinding == rebinding {
                    Color::ORANGE
                } else {
                    Color::PURPLE
                },
            );

            d.draw_text(
                &Controls::key_to_string(key),
                (rect.x + 35.0) as i32,
                (rect.y + 15.0) as i32,
                20,
                Color::BLACK,
            );
        }
        d.draw_text(
            "Press Backspace to return to main menu...",
            ((controls.window_width as f32 / 2.0) - 200.0) as i32,
            ((controls.window_height as f32 / 2.0) + 120.0) as i32,
            20,
            Color::BLACK,
        );

        if controls.display_rebind_err {
            d.draw_text(
                "Key already in use! Try another key.",
                ((controls.window_width as f32 / 2.0) - 200.0) as i32,
                ((controls.window_height as f32 / 2.0) + 160.0) as i32,
                20,
                Color::RED,
            );
        }
    }
}
