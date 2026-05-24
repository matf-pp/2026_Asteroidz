use raylib::prelude::*;
use crate::controls::Controls;
use crate::ThrusterState;
use crate::GameScreen;
pub struct AudioManager{
    audio_device: RaylibAudio,
    background_music: Music,
    sfx_thruster: Music,
    is_thruster_sfx_playing: bool,
    laser_pool: [Sound; 5],
    current_laser: usize,
    volume: f32,
    muted: bool,
}

impl AudioManager{
    pub fn new (thread: &RaylibThread) -> Self{

        let mut audio_device = RaylibAudio::init_audio_device();

        let mut background_music =
        Music::load_music_stream(&thread, "assets/test_background.mp3").unwrap();
        background_music.looping = true;
        audio_device.play_music_stream(&mut background_music);

        let mut sfx_thruster = Music::load_music_stream(&thread, "assets/test_thruster.mp3").unwrap();
        sfx_thruster.looping = true;

        let laser_pool = [
            Sound::load_sound("assets/test_laser.wav").unwrap(),
            Sound::load_sound("assets/test_laser.wav").unwrap(),
            Sound::load_sound("assets/test_laser.wav").unwrap(),
            Sound::load_sound("assets/test_laser.wav").unwrap(),
            Sound::load_sound("assets/test_laser.wav").unwrap(),
        ]; //doesn't seem to be any better solution to allow overlapping

        let volume = 1.0;
        audio_device.set_master_volume(volume);

        Self{
            audio_device,
            background_music,
            sfx_thruster,
            is_thruster_sfx_playing:false,
            laser_pool,
            current_laser:0,
            volume,
            muted:false,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, controls: &Controls, thruster_state: &ThrusterState, active_screen: &GameScreen){
        self.audio_device.update_music_stream(&mut self.background_music);
        if rl.is_key_pressed(controls.mute) {
            self.audio_device.set_master_volume(if !self.muted { 0.0 } else { self.volume });
            self.muted = !self.muted;
        }
        if rl.is_key_down(controls.volume_up) {
            self.volume = (self.volume + 0.01).min(1.0);
            if !self.muted {
                self.audio_device.set_master_volume(self.volume);
            }
        }
        if rl.is_key_down(controls.volume_down) {
            self.volume = (self.volume - 0.01).max(0.0);
            if !self.muted {
                self.audio_device.set_master_volume(self.volume);
            }
        }
        if *active_screen == GameScreen::Paused
        {
            if self.is_thruster_sfx_playing{
                self.audio_device.stop_music_stream(&mut self.sfx_thruster);
                self.is_thruster_sfx_playing=false;
            }
        }
        else{
            if *thruster_state != ThrusterState::Off {
                self.audio_device.update_music_stream(&mut self.sfx_thruster);
                if !self.is_thruster_sfx_playing {
                    self.audio_device.play_music_stream(&mut self.sfx_thruster);
                    self.is_thruster_sfx_playing = true;
                }
            } else {
                if self.is_thruster_sfx_playing {
                    self.audio_device.stop_music_stream(&mut self.sfx_thruster);
                    self.is_thruster_sfx_playing = false;
                }
            }
        }
    }

    pub fn play_laser(&mut self) {
        self.audio_device.play_sound(&self.laser_pool[self.current_laser]);
        self.current_laser = (self.current_laser + 1) % self.laser_pool.len();
    }
}