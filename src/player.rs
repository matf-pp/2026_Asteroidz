use crate::controls::Controls;
use raylib::prelude::*;

#[derive(PartialEq)]

pub enum ThrusterState {
    Off,
    Single,
    Triple,
}

pub struct Player {
    pub position: Vector2,
    pub velocity: Vector2,
    pub box_size: Vector2,
    pub angle: f32,
    pub thruster_state: ThrusterState,
    pub thruster_timer: f32,
    pub health: u8,
    pub invincible_timer: f32,
    pub visible: bool,
    pub proj_delay: f32,
}

impl Player {
    const SPEED: f32 = 1.00;
    const ROTATION_SPEED: f32 = 2.5;
    const ANIMATION_SPEED: f32 = 0.1;
    const PROJ_DELAY: f32 = 0.5;
    pub fn new(
        position: Vector2,
        velocity: Vector2,
        box_size: Vector2,
        angle: f32,
        thruster_state: ThrusterState,
    ) -> Self {
        Self {
            position,
            velocity,
            box_size,
            angle,
            thruster_state,
            thruster_timer: 0.0,
            health: 3,
            invincible_timer: 0.0,
            visible: true,
            proj_delay: 0.0,
        }
    }
    pub fn update(
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
            self.visible =
                (self.invincible_timer <= 0.0) || ((self.invincible_timer * 5.0) as i32) % 2 == 0;
            // when timer runs out, ship MUST be visible, otherwise blinking...
        }

        if self.proj_delay > 0.0 {
            self.proj_delay -= dt;
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

    pub fn take_damage(&mut self) {
        if self.invincible_timer <= 0.0 {
            self.health = self.health.saturating_sub(1);
            self.invincible_timer = 2.0; // ship is invincible for 2s after hit
        }
    }

    pub fn is_alive(&self) -> bool {
        return self.health > 0;
    }

    pub fn reset_proj_timer(&mut self) {
        self.proj_delay = Player::PROJ_DELAY;
    }
}
