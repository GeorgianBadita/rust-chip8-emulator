use crate::{constants, keys};
use rand::random;
use sdl2::{event::Event, keyboard::Scancode, EventPump};
use std::{cell::RefMut, fs::File, io::Read, process};

pub struct Chip8 {
    memory: [u8; constants::MEMORY_IN_B],
    registers: [u8; constants::NUM_REGISTERS],
    index_register: u16,
    program_counter: u16,
    screen: [u8; constants::SCREEN_SIZE],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; constants::STACK_LEVELS],
    stack_pointer: u16,

    // Emulation speed [ns]
    instruction_speed: u128,
    // Last timer time
    last_timer_time: u128,
    // Last instruction time
    last_instruction_time: u128,
    // Flag for clearing screen
    clear_screen_flag: bool,
    // Flag for updating screen
    update_screen_flag: bool,
    // Flag for beep sound
    beep_sound_flag: bool,
    // Debug flag
    debug: bool,
}

impl Chip8 {
    pub fn new(
        rom_file_path: &str,
        instructions_per_second: u128,
        start_time: u128,
        debug: bool,
    ) -> Self {
        // RAM Memory
        let mut memory = [0; constants::MEMORY_IN_B];
        memory[..80].copy_from_slice(&[
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ]);
        print!("Fonts loaded in memory");

        // ROM
        let rom = Self::read_rom(rom_file_path);
        // Registers
        let registers = [0; constants::NUM_REGISTERS];
        // Index register
        let index_register = 0;
        // Program counter
        let program_counter = constants::PROGRAM_MEMORY_START as u16;
        // Screen array
        let screen = [0; constants::SCREEN_SIZE];
        // Delay timer
        let delay_timer = 0;
        // Sound timer
        let sound_timer = 0;
        // Stack
        let stack = [0; constants::STACK_LEVELS];
        // Stack pointer
        let stack_pointer = 0;
        // Instruction speed [ns]
        let instruction_speed = 1e9 as u128 / instructions_per_second as u128;
        // Last timer time
        let last_timer_time = start_time;
        // Last timer time
        let last_instruction_time = start_time;

        let rom_size = rom.len();
        memory[constants::PROGRAM_MEMORY_START..constants::PROGRAM_MEMORY_START + rom_size]
            .copy_from_slice(&rom[0..rom_size]);

        Chip8 {
            memory,
            registers,
            index_register,
            program_counter,
            screen,
            delay_timer,
            sound_timer,
            stack,
            stack_pointer,
            instruction_speed,
            last_timer_time,
            last_instruction_time,
            clear_screen_flag: false,
            update_screen_flag: false,
            beep_sound_flag: false,
            debug,
        }
    }

    pub fn get_screen(&self) -> &[u8; constants::SCREEN_SIZE] {
        &self.screen
    }

    pub fn should_beep(&self) -> bool {
        self.beep_sound_flag
    }

    pub fn should_clear_screen(&self) -> bool {
        self.clear_screen_flag
    }

    pub fn should_update_screen(&self) -> bool {
        self.update_screen_flag
    }

    pub fn cycle(&mut self, current_time: u128, mut event_pump: RefMut<EventPump>) {
        self.update_screen_flag = false;
        self.clear_screen_flag = false;

        // Update timers 60 times / s
        if current_time - self.last_timer_time >= 16_666_666 {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
                self.beep_sound_flag = self.sound_timer > 0;
            }
            self.last_timer_time = current_time;
        }

        // Interpret instruction
        if current_time - self.last_instruction_time >= self.instruction_speed {
            if self.program_counter as usize >= constants::MEMORY_IN_B {
                panic!("Reached end of the program!");
            }

            let instruction = (self.memory[self.program_counter as usize] as u16) << 8
                | self.memory[self.program_counter as usize + 1] as u16;
            self.program_counter += 2;

            let code = instruction & 0xF000;
            let x = ((instruction & 0x0F00) >> 8) as usize;
            let y = ((instruction & 0x00F0) >> 4) as usize;
            let n = instruction & 0x000F;
            let nn = instruction & 0x00FF;
            let nnn = instruction & 0x0FFF;

            match code {
                0x0000 => match n {
                    0 => {
                        self.screen.iter_mut().for_each(|val| *val = 0);
                        self.clear_screen_flag = true;
                    }
                    0xE => {
                        self.program_counter = self.stack[self.stack_pointer as usize];
                        if self.stack_pointer > 0 {
                            self.stack_pointer -= 1;
                        }
                    }
                    _ => {}
                },
                0x1000 => self.program_counter = nnn,
                0x2000 => {
                    self.index_register += 1;
                    self.stack[self.stack_pointer as usize] = self.program_counter;
                    self.program_counter = nnn;
                }
                0x3000 => {
                    if self.registers[x] as u16 == nn {
                        self.program_counter += 2;
                    }
                }
                0x4000 => {
                    if self.registers[x] as u16 != nn {
                        self.program_counter += 2;
                    }
                }
                0x5000 => {
                    if self.registers[x] == self.registers[y] {
                        self.program_counter += 2;
                    }
                }
                0x6000 => self.registers[x] = nn as u8,
                0x7000 => self.registers[x] = (self.registers[x] as u16 + nn) as u8,
                0x8000 => match n {
                    0x00 => self.registers[x] = self.registers[y],
                    0x01 => self.registers[x] = self.registers[x] | self.registers[y],
                    0x02 => self.registers[x] = self.registers[x] & self.registers[y],
                    0x03 => self.registers[x] = self.registers[x] ^ self.registers[y],
                    0x04 => {
                        let result: usize = self.registers[x] as usize + self.registers[y] as usize;
                        if result > 255 {
                            self.registers[0x0F] = 1;
                        } else {
                            self.registers[0x0F] = 0;
                        }
                        self.registers[x] = result as u8;
                    }
                    0x05 => {
                        self.registers[0x0F] = if self.registers[x] > self.registers[y] {
                            1
                        } else {
                            0
                        };
                        self.registers[x] =
                            (self.registers[x] as i32 - self.registers[y] as i32) as u8;
                    }
                    0x06 => {
                        self.registers[0x0F] = self.registers[x] & 0x01;
                        self.registers[x] /= 2;
                    }
                    0x07 => {
                        self.registers[0x0F] = if self.registers[y] > self.registers[x] {
                            1
                        } else {
                            0
                        };
                        self.registers[x] =
                            (self.registers[y] as i32 - self.registers[x] as i32) as u8;
                    }
                    0x0E => {
                        self.registers[0x0F] = self.registers[x] & 0x080;
                        self.registers[x] = (self.registers[x] as u16 * 2) as u8;
                    }
                    _ => {}
                },
                0x9000 => {
                    if self.registers[x] != self.registers[y] {
                        self.program_counter += 2;
                    }
                }
                0xA000 => self.index_register = nnn,
                0xB000 => self.program_counter = nnn + self.registers[0] as u16,
                0xC000 => self.registers[x] = nn as u8 & random::<u8>(),
                0xD000 => {
                    self.registers[0xF] = 0;
                    let xpos: usize = self.registers[x] as usize % constants::SCREEN_WIDTH;
                    let ypos: usize = self.registers[y] as usize % constants::SCREEN_HEIGHT;
                    for row in 0..n {
                        let byte = self.memory[(self.index_register as usize + row as usize)] as u8;
                        let current_y = (ypos + row as usize) % constants::SCREEN_HEIGHT;

                        for col in 0..8 {
                            let current_x = (xpos + col) % constants::SCREEN_WIDTH;
                            let current_value =
                                self.screen[current_y * constants::SCREEN_WIDTH + current_x];
                            let mask: u8 = 0x01 << 7 - col;
                            let color = byte & mask;

                            if color > 0 {
                                if current_value > 0 {
                                    self.screen[current_y * constants::SCREEN_WIDTH + current_x] =
                                        0;
                                    self.registers[0x0F] = 1;
                                } else {
                                    self.screen[current_y * constants::SCREEN_WIDTH + current_x] =
                                        1;
                                }
                            }
                            if current_x == constants::SCREEN_WIDTH - 1 {
                                break;
                            }
                        }
                        if current_y == constants::SCREEN_HEIGHT - 1 {
                            break;
                        }
                    }
                    self.update_screen_flag = true;
                }
                0xE000 => match nn {
                    0x9E => {
                        if event_pump
                            .keyboard_state()
                            .is_scancode_pressed(keys::map(self.registers[x]))
                        {
                            self.program_counter += 2;
                        }
                    }
                    0xA1 => {
                        if !event_pump
                            .keyboard_state()
                            .is_scancode_pressed(keys::map(self.registers[x]))
                        {
                            self.program_counter += 2;
                        }
                    }
                    _ => (),
                },
                0xF000 => match nn {
                    0x07 => self.registers[x] = self.delay_timer,
                    0x0A => {
                        let keycode: u8 = loop {
                            let event = event_pump.wait_event();
                            let code = match event {
                                Event::KeyDown {
                                    keycode: Some(code),
                                    ..
                                } => Some(code),
                                _ => None,
                            };
                            if code.is_some() {
                                let sc = Scancode::from_keycode(code.unwrap()).unwrap();
                                if sc == Scancode::Escape || sc == Scancode::CapsLock {
                                    process::exit(0);
                                }
                                let c = keys::unmap(sc);
                                if c.is_some() {
                                    break c.unwrap();
                                }
                            }
                        };
                        self.registers[x] = keycode;
                    }
                    0x15 => self.delay_timer = self.registers[x],
                    0x18 => self.sound_timer = self.registers[x],
                    0x1E => self.index_register = self.index_register + self.registers[x] as u16,
                    0x29 => self.index_register = self.registers[x] as u16 * 0x05,
                    0x33 => {
                        let num = self.registers[x];
                        let h = num / 100;
                        let t = (num - h * 100) / 10;
                        let o = num - h * 100 - t * 10;
                        let i = self.index_register as usize;
                        self.memory[i] = h;
                        self.memory[i + 1] = t;
                        self.memory[i + 2] = o;
                    }
                    0x55 => {
                        let n: usize = x;
                        for reg in 0..n + 1 {
                            self.memory[self.index_register as usize + reg] = self.registers[reg];
                        }
                    }
                    0x65 => {
                        let n: usize = x;
                        for reg in 0..n + 1 {
                            self.registers[reg] = self.memory[self.index_register as usize + reg];
                        }
                    }
                    _ => (),
                },
                _ => {}
            }
        }
    }

    fn read_rom(rom_file_path: &str) -> Vec<u8> {
        let mut rom = Vec::new();
        File::open(rom_file_path)
            .unwrap()
            .read_to_end(&mut rom)
            .unwrap();
        rom
    }
}
