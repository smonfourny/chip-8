use crate::opcode::OpCode;
use crate::disassembler::Disassembler;
use std::io;
use crate::constants::{PC_DEFAULT_START, FONT, FONT_START};


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

        Self::initialize_program(&mut memory, &program);
        Self::initialize_font(&mut memory);

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

    fn handle_op(&mut self, op_code: &OpCode) {
        let nibble = op_code.first >> 4 & 0xF;
        match nibble {
            0x0 => todo!(),
            0x1 => todo!(),
            0x2 => todo!(),
            0x3 => todo!(),
            0x4 => todo!(),
            0x5 => todo!(),
            0x6 => todo!(),
            0x7 => todo!(),
            0x8 => todo!(),
            0x9 => todo!(),
            0xa => self.handle_a_op(op_code),
            0xb => todo!(),
            0xc => todo!(),
            0xd => todo!(),
            0xe => todo!(),
            0xf => todo!(),
            _ => panic!("impossible!"),
        }
    }

    fn read_op_codes(&self) -> io::Result<Vec<OpCode>> {
        let res = self.memory[PC_DEFAULT_START..PC_DEFAULT_START + self.program_length]
            .iter()
            .enumerate()
            .step_by(2)
            .map(|(i, byte)| { OpCode { first: *byte, second: self.memory[PC_DEFAULT_START+i+1] } })
            .collect();

        Ok(res)
    }

    pub fn disassemble_program(&self) {
        let op_codes = self.read_op_codes().unwrap();

        for op_code in op_codes {
            self.disassembler.handle_op(&op_code);
        }
    }

    fn initialize_program(memory: &mut [u8; 4096], program: &Vec<u8>) {
        for (i, byte) in program.iter().enumerate() {
            memory[0x200 + i] = *byte;
        }
    }

    fn initialize_font(memory: &mut [u8; 4096]) {
        for (i, val) in FONT.iter().enumerate() {
            memory[FONT_START + i] = *val;
        }
    }

    fn handle_a_op(&mut self, op_code: &OpCode) {
        self.i = op_code.to_u16() & 0xfff;
    }
}