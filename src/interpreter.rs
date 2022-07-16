use crate::opcode::OpCode;
use crate::disassembler::Disassembler;
use std::io;
use crate::constants::{PC_DEFAULT_START, FONT, FONT_START, DISPLAY_MEM_START};
use crate::util::get_bit_at;


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
            pc: PC_DEFAULT_START as u16,
            memory,
            program_length: program.len(),
            disassembler: Disassembler {}
        }
    }

    pub fn tick(&mut self) {
        let op_code = OpCode { first: self.memory[self.pc as usize], second: self.memory[(self.pc+1) as usize] };
        self.handle_op(&op_code);
    }

    fn handle_op(&mut self, op_code: &OpCode) {
        let nibble = op_code.first >> 4 & 0xF;
        match nibble {
            0x0 => self.handle_0_op(op_code),
            0x1 => self.disassembler.handle_op(op_code),
            0x2 => self.disassembler.handle_op(op_code),
            0x3 => self.disassembler.handle_op(op_code),
            0x4 => self.disassembler.handle_op(op_code),
            0x5 => self.disassembler.handle_op(op_code),
            0x6 => self.handle_6_op(op_code),
            0x7 => self.disassembler.handle_op(op_code),
            0x8 => self.disassembler.handle_op(op_code),
            0x9 => self.disassembler.handle_op(op_code),
            0xa => self.handle_a_op(op_code),
            0xb => self.disassembler.handle_op(op_code),
            0xc => self.disassembler.handle_op(op_code),
            0xd => self.handle_d_op(op_code),
            0xe => self.disassembler.handle_op(op_code),
            0xf => self.disassembler.handle_op(op_code),
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

    fn clear_screen(&mut self) {
        for i in 0xf00..0xfff {
            self.memory[i] = 0;
        }
    }

    fn handle_0_op(&mut self, op_code: &OpCode) {
        match op_code.first & 0xF {
            0 => match op_code.second {
                0xe0 => self.clear_screen(),
                0xee => panic!("Not implemented!"),
                _ => panic!("Unknown op code"),
            },
            _ => panic!("Not implemented!"),
        }
        self.pc += 2;
    }

    fn handle_6_op(&mut self, op_code: &OpCode) {
        let register = (op_code.first & 0xF) as usize;
        self.v[register] = op_code.second;
        self.pc += 2;
    }

    fn handle_a_op(&mut self, op_code: &OpCode) {
        self.i = op_code.to_u16() & 0xfff;
        self.pc += 2;
    }

    fn handle_d_op(&mut self, op_code: &OpCode) {
        // TODO implement vf behaviour
        let flipped = 0;
        let n = op_code.second & 0xF;
        let x = self.v[(op_code.first & 0xF) as usize];
        let y = self.v[(op_code.second >> 4 & 0xF) as usize];

        for j in 0..n {
            let pos = x as u16 + 64 * (y as u16 + j as u16);

            let sprite_line = self.memory[(self.i + j as u16) as usize];

            // display mem location pointers
            let mut mem_loc = DISPLAY_MEM_START + (pos / 8) as usize;
            let mut bit_loc = pos % 8;

            // For each bit of sprite
            for k in 0..8 {
                let bit = if get_bit_at(sprite_line, k) { 1 } else { 0 };

                self.memory[mem_loc] = self.memory[mem_loc] | (bit << bit_loc);

                bit_loc += 1;
                if bit_loc >= 8 {
                    mem_loc += 1;
                    bit_loc = 0;
                }
            }
        }

        self.v[0xF] = flipped;
        self.pc += 2;
    }
}