use std::cell::RefCell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{chip8, media};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Emulator {
    // Sdl Screen
    sdl_screen: Rc<RefCell<media::Screen>>,
    // Sdl Beep
    sdl_beep: media::Beep,
    // Chip 8
    chip8: chip8::Chip8,
}

impl Emulator {
    pub fn new(
        title: &'static str,
        rom_path: &str,
        scale: u32,
        emulation_instr_second: u128,
        debug: bool,
    ) -> Self {
        // Sdl Context
        let sdl_context = sdl2::init().unwrap();
        // Sdl Screen
        let sdl_screen = media::Screen::new(&sdl_context, title, scale);
        // Sdl Beep
        let sdl_beep = media::Beep::new(&sdl_context);
        // Chip8
        let chip8 = chip8::Chip8::new(rom_path, emulation_instr_second, Self::nanos_time(), debug);

        Emulator {
            sdl_screen: Rc::new(RefCell::new(sdl_screen)),
            sdl_beep,
            chip8,
        }
    }

    pub fn emulate(&mut self) {
        'mainloop: loop {
            let current_time = Self::nanos_time();
            let screen = Rc::clone(&self.sdl_screen);
            let screen_ref = screen.as_ref();

            for event in screen_ref.borrow().event_pump.borrow_mut().poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'mainloop,
                    _ => {}
                }
            }

            // Run a cycle
            self.chip8
                .cycle(current_time, screen_ref.borrow().event_pump.borrow_mut());

            // Change display
            if self.chip8.should_clear_screen() {
                screen_ref.borrow_mut().clear_screen();
            }
            if self.chip8.should_update_screen() {
                screen_ref
                    .borrow_mut()
                    .update_screen(self.chip8.get_screen());
            }

            // Audio media
            if self.chip8.should_beep() {
                self.sdl_beep.play();
            } else {
                self.sdl_beep.pause();
            }
        }
    }

    fn nanos_time() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    }
}
