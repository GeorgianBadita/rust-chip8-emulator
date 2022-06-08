use std::cell::RefCell;

use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired, AudioStatus};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::Sdl;
use sdl2::{render::Canvas, video::Window, EventPump};

use crate::constants;

// SDL Screen for emulator
pub struct Screen {
    canvas: Canvas<Window>,
    pub event_pump: RefCell<EventPump>,
    scale: u32,
}

impl Screen {
    const BLACK_COLOR: Color = Color::RGB(255, 255, 255);
    const WHITE_COLOR: Color = Color::RGB(0, 0, 0);

    pub fn new(sdl: &Sdl, title: &str, scale: u32) -> Self {
        let video_subsystem = sdl.video().unwrap();
        let event_pump = sdl.event_pump().unwrap();

        let window = video_subsystem
            .window(
                title,
                constants::SCREEN_WIDTH as u32 * scale,
                constants::SCREEN_HEIGHT as u32 * scale,
            )
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        Screen {
            canvas,
            event_pump: RefCell::new(event_pump),
            scale,
        }
    }

    // Clears the display
    pub fn clear_screen(&mut self) {
        self.canvas.set_draw_color(Self::BLACK_COLOR);
        self.canvas.clear();
        self.canvas.present();
    }

    // Updates screen with buffer
    pub fn update_screen(&mut self, buffer: &[u8; constants::SCREEN_SIZE]) {
        let scale = self.scale as usize;

        for x in 0..constants::SCREEN_WIDTH {
            for y in 0..constants::SCREEN_HEIGHT {
                match buffer[y * constants::SCREEN_WIDTH + x] > 0 {
                    true => self.canvas.set_draw_color(Self::BLACK_COLOR),
                    false => self.canvas.set_draw_color(Self::WHITE_COLOR),
                }
                self.canvas
                    .fill_rect(Rect::new(
                        (x * scale) as i32,
                        (y * scale) as i32,
                        scale as u32,
                        scale as u32,
                    ))
                    .unwrap();
            }
        }
        self.canvas.present();
    }
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}
impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

// SDL Beep for emulator
pub struct Beep {
    device: AudioDevice<SquareWave>,
}

impl Beep {
    pub fn new(sdl_context: &Sdl) -> Self {
        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1), // mono
            samples: None,     // default sample size
        };

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| {
                // initialize the audio callback
                SquareWave {
                    phase_inc: 440.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.2,
                }
            })
            .unwrap();

        Beep { device }
    }

    pub fn play(&self) {
        if self.device.status() == AudioStatus::Stopped
            || self.device.status() == AudioStatus::Paused
        {
            self.device.resume();
        }
    }

    pub fn pause(&self) {
        self.device.pause();
    }
}
