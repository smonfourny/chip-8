pub struct OpCode {
    pub first: u8,
    pub second: u8,
}

impl OpCode {
    pub fn to_u16(&self) -> u16 {
        ((self.first as u16) << 8) | self.second as u16
    }
}
