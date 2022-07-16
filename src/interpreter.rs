use crate::types::{OpCode, HandleOp};
use crate::disassembler::Disassembler;
use std::io;

pub struct Interpreter {
    v: [u8; 16], // general purpose registers
    i: u16, // I register, 12-bit wide
    dt: u8, // timer register
    st: u8, // sound time register
    sp: u8, // stack pointer
    pc: u16, // program counter
    pub memory: [u8; 4096], // RAM
    program_length: usize,
    disassembler: Disassembler,
}

impl Interpreter {
    pub fn new(program: Vec<u8>) -> Self {
        let mut memory = [0; 4096];

        for (i, byte) in program.iter().enumerate() {
            memory[0x200 + i] = *byte;
        }

        Interpreter {
            v: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
            sp: 0,
            pc: 0x200,
            memory,
            program_length: program.len(),
            disassembler: Disassembler {}
        }
    }

    fn read_op_codes(&self) -> io::Result<Vec<OpCode>> {
        let res = self.memory[0x200..0x200 + self.program_length]
            .iter()
            .enumerate()
            .step_by(2)
            .map(|(i, byte)| { OpCode { first: *byte, second: self.memory[0x200+i+1] } })
            .collect();

        Ok(res)
    }

    pub fn disassemble_program(&self) {
        let op_codes = self.read_op_codes().unwrap();

        for op_code in op_codes {
            self.disassembler.handle_op(&op_code);
        }
    }
}