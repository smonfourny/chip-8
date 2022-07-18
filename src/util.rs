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
    pub wait_for_keyboard: bool,
}

pub fn get_bit_at(input: u8, n: u8) -> bool {
    if n < 8 {
        input & (1 << n) != 0
    } else {
        false
    }
}
