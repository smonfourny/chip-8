use winit::event::VirtualKeyCode;

pub struct OpCode {
    pub first: u8,
    pub second: u8,
}

impl OpCode {
    pub fn to_u16(&self) -> u16 {
        ((self.first as u16) << 8) | self.second as u16
    }
}

pub struct InterpreterResult {
    pub refresh_display: bool,
    pub wait_for_keyboard: Option<usize>,
}

pub fn key_to_chip_8(key: VirtualKeyCode) -> u8 {
    match key {
        VirtualKeyCode::Key1 => 0,
        VirtualKeyCode::Key2 => 1,
        VirtualKeyCode::Key3 => 2,
        VirtualKeyCode::Key4 => 3,
        VirtualKeyCode::Q => 4,
        VirtualKeyCode::W => 5,
        VirtualKeyCode::E => 6,
        VirtualKeyCode::R => 7,
        VirtualKeyCode::A => 8,
        VirtualKeyCode::S => 9,
        VirtualKeyCode::D => 10,
        VirtualKeyCode::F => 11,
        VirtualKeyCode::Z => 12,
        VirtualKeyCode::X => 13,
        VirtualKeyCode::C => 14,
        VirtualKeyCode::V => 15,
        _ => panic!("Unknown key!")
    }
}

pub fn get_bit_at(input: u8, n: u8) -> bool {
    if n < 8 {
        input & (1 << n) != 0
    } else {
        false
    }
}
