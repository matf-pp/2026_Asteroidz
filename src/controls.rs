use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
pub struct Controls {
    pub left: KeyboardKey,
    pub right: KeyboardKey,
    pub forward: KeyboardKey,
    pub shoot: KeyboardKey,
    pub mute: KeyboardKey,
    pub volume_up: KeyboardKey,
    pub volume_down: KeyboardKey,
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
    ) -> Self {
        Self {
            left: left.unwrap_or(KEY_LEFT),
            right: right.unwrap_or(KEY_RIGHT),
            forward: forward.unwrap_or(KEY_UP),
            shoot: shoot.unwrap_or(KEY_S),
            mute: mute.unwrap_or( KEY_M),
            volume_up: volume_up.unwrap_or(KEY_F3),
            volume_down: volume_down.unwrap_or(KEY_F2),
        }
    }
}
