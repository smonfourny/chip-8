pub struct Interpreter {
    v: [u8; 16], // general purpose registers
    i: u16, // I register, 12-bit wide
    dt: u8, // timer register
    st: u8, // sound time register
    sp: u8, // stack pointer
    pc: u16, // program counter
    memory: [u8; 4096] // RAM
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            v: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
            sp: 0,
            pc: 0x200,
            memory: [0; 4096]
        }
    }
}