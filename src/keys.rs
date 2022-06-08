use sdl2::keyboard::Scancode;

// Converts bytes into scan codes
// The mapping is done with the following keys:
// 1 2 3 C      1 2 3 4
// 4 5 6 D      Q W E R
// 7 8 9 E  =>  A S D F
// A 0 B F      Z X C V
pub fn map(code: u8) -> Scancode {
    match code {
        0x00 => Scancode::X,
        0x01 => Scancode::Num1,
        0x02 => Scancode::Num2,
        0x03 => Scancode::Num3,
        0x04 => Scancode::Q,
        0x05 => Scancode::W,
        0x06 => Scancode::E,
        0x07 => Scancode::A,
        0x08 => Scancode::S,
        0x09 => Scancode::D,
        0x0A => Scancode::Z,
        0x0B => Scancode::C,
        0x0C => Scancode::Num4,
        0x0D => Scancode::R,
        0x0E => Scancode::F,
        0x0F => Scancode::V,
        _ => Scancode::Escape,
    }
}

pub fn unmap(scancode: Scancode) -> Option<u8> {
    match scancode {
        Scancode::X => Some(0x00),
        Scancode::Num1 => Some(0x01),
        Scancode::Num2 => Some(0x02),
        Scancode::Num3 => Some(0x03),
        Scancode::Q => Some(0x04),
        Scancode::W => Some(0x05),
        Scancode::E => Some(0x06),
        Scancode::A => Some(0x07),
        Scancode::S => Some(0x08),
        Scancode::D => Some(0x09),
        Scancode::Z => Some(0x0A),
        Scancode::C => Some(0x0B),
        Scancode::Num4 => Some(0x0C),
        Scancode::R => Some(0x0D),
        Scancode::F => Some(0x0E),
        Scancode::V => Some(0x0F),
        _ => None,
    }
}
