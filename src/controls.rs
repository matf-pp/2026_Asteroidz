use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
pub struct Controls {
    pub left: KeyboardKey,
    pub right: KeyboardKey,
    pub forward: KeyboardKey,
    pub shoot: KeyboardKey,
}

impl Controls {
    pub fn new(
        left: Option<KeyboardKey>,
        right: Option<KeyboardKey>,
        forward: Option<KeyboardKey>,
        shoot: Option<KeyboardKey>,
    ) -> Self {
        Self {
            left: left.unwrap_or(KEY_LEFT),
            right: right.unwrap_or(KEY_RIGHT),
            forward: forward.unwrap_or(KEY_UP),
            shoot: shoot.unwrap_or(KEY_S),
        }
    }
}
