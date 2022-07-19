use crate::constants::{DISPLAY_MEM_START, FONT, FONT_START, PC_DEFAULT_START, STACK_START};
use crate::disassembler::Disassembler;
use crate::util::get_bit_at;
use crate::util::{InterpreterResult, OpCode};
use rand::random;
use std::io;

pub struct Interpreter {
    v: [u8; 16],            // general purpose registers
    i: u16,                 // I register, 12-bit wide
    dt: u8,                 // timer register
    st: u8,                 // sound time register
    sp: u8,                 // stack pointer
    pc: u16,                // program counter
    pub memory: [u8; 4096], // RAM
    keyboard: [bool; 16],
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
            sp: STACK_START as u8,
            pc: PC_DEFAULT_START as u16,
            memory,
            program_length: program.len(),
            keyboard: [false; 16],
            disassembler: Disassembler {},
        }
    }

    pub fn tick(&mut self) -> InterpreterResult {
        let op_code = OpCode {
            first: self.memory[self.pc as usize],
            second: self.memory[(self.pc + 1) as usize],
        };
        if self.dt > 0 {
            self.dt -= 1
        }
        if self.st > 0 {
            // TODO beep somehow
            self.st -= 1
        }
        self.handle_op(&op_code)
    }

    pub fn press_key(&mut self, key: u8, pressed: bool) {
        self.keyboard[key as usize] = pressed;
    }

    pub fn store_key(&mut self, register: usize, key: u8) {
        self.v[register] = key;
    }

    fn handle_op(&mut self, op_code: &OpCode) -> InterpreterResult {
        let nibble = op_code.first >> 4 & 0xF;
        match nibble {
            0x0 => self.handle_0_op(op_code),
            0x1 => self.handle_1_op(op_code),
            0x2 => self.handle_2_op(op_code),
            0x3 => self.handle_3_op(op_code),
            0x4 => self.handle_4_op(op_code),
            0x5 => self.handle_5_op(op_code),
            0x6 => self.handle_6_op(op_code),
            0x7 => self.handle_7_op(op_code),
            0x8 => self.handle_8_op(op_code),
            0x9 => self.handle_9_op(op_code),
            0xa => self.handle_a_op(op_code),
            0xb => self.handle_b_op(op_code),
            0xc => self.handle_c_op(op_code),
            0xd => self.handle_d_op(op_code),
            0xe => self.handle_e_op(op_code),
            0xf => self.handle_f_op(op_code),
            _ => panic!("impossible!"),
        }
    }

    fn read_op_codes(&self) -> io::Result<Vec<OpCode>> {
        let res = self.memory[PC_DEFAULT_START..PC_DEFAULT_START + self.program_length]
            .iter()
            .enumerate()
            .step_by(2)
            .map(|(i, byte)| OpCode {
                first: *byte,
                second: self.memory[PC_DEFAULT_START + i + 1],
            })
            .collect();

        Ok(res)
    }

    pub fn disassemble_program(&self) {
        let op_codes = self.read_op_codes().unwrap();

        for op_code in op_codes {
            self.disassembler.handle_op(&op_code);
        }
    }

    fn initialize_program(memory: &mut [u8; 4096], program: &[u8]) {
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

    fn handle_0_op(&mut self, op_code: &OpCode) -> InterpreterResult {
        match op_code.first & 0xF {
            0 => match op_code.second {
                0xe0 => {
                    self.clear_screen();
                    self.pc += 2;
                    InterpreterResult {
                        refresh_display: true,
                        wait_for_keyboard: None,
                    }
                }
                0xee => {
                    let left = self.memory[(self.sp - 2) as usize];
                    let right = self.memory[(self.sp - 1) as usize];
                    self.sp -= 2;
                    self.pc = (left as u16) << 8 | right as u16;
                    InterpreterResult {
                        refresh_display: false,
                        wait_for_keyboard: None,
                    }
                }
                _ => panic!("Unknown op code"),
            },
            _ => panic!("Not implemented!"),
        }
    }

    fn handle_1_op(&mut self, op_code: &OpCode) -> InterpreterResult {
        self.pc = op_code.to_u16() & 0xfff;
        InterpreterResult {
            refresh_display: false,
            wait_for_keyboard: None,
        }
    }

    fn handle_2_op(&mut self, op_code: &OpCode) -> InterpreterResult {
        let n = op_code.to_u16() & 0xfff;

        let next_pc = self.pc + 2;
        let next_pc_l = next_pc >> 8;
        let next_pc_r = next_pc & 0x00FF;

        self.memory[self.sp as usize] = next_pc_l as u8;
        self.memory[(self.sp + 1) as usize] = next_pc_r as u8;
        self.sp += 2;

        self.pc = n;
        InterpreterResult {
            refresh_display: false,
            wait_for_keyboard: None,
        }
    }

    fn handle_3_op(&mut self, op_code: &OpCode) -> InterpreterResult {
        let register = (op_code.first & 0xF) as usize;
        let n = op_code.second;
        if self.v[register] == n {
            self.pc += 2;
        }
        self.pc += 2;
        InterpreterResult {
            refresh_display: false,
            wait_for_keyboard: None,
        }
    }

    fn handle_4_op(&mut self, op_code: &OpCode) -> InterpreterResult {
        let register = (op_code.first & 0xF) as usize;
        let n = op_code.second;
        if self.v[register] != n {
            self.pc += 2;
        }
        self.pc += 2;
        InterpreterResult {
            refresh_display: false,
            wait_for_keyboard: None,
        }
    }

    fn handle_5_op(&mut self, op_code: &OpCode) -> InterpreterResult {
        let register_1 = (op_code.first & 0xF) as usize;
        let register_2 = (op_code.second >> 4 & 0xF) as usize;
        if self.v[register_1] == self.v[register_2] {
            self.pc += 2;
        }
        self.pc += 2;
        InterpreterResult {
            refresh_display: false,
            wait_for_keyboard: None,
        }
    }

    fn handle_6_op(&mut self, op_code: &OpCode) -> InterpreterResult {
        let register = (op_code.first & 0xF) as usize;
        self.v[register] = op_code.second;
        self.pc += 2;
        InterpreterResult {
            refresh_display: false,
            wait_for_keyboard: None,
        }
    }

    fn handle_7_op(&mut self, op_code: &OpCode) -> InterpreterResult {
        let register = (op_code.first & 0xF) as usize;
        self.v[register] = u8::wrapping_add(self.v[register], op_code.second);
        self.pc += 2;
        InterpreterResult {
            refresh_display: false,
            wait_for_keyboard: None,
        }
    }

    fn handle_8_op(&mut self, op_code: &OpCode) -> InterpreterResult {
        match op_code.second & 0xF {
            0x0 => {
                let register_1 = (op_code.first & 0xF) as usize;
                let register_2 = (op_code.second >> 4 & 0xF) as usize;
                self.v[register_1] = self.v[register_2];
            }
            0x1 => {
                let register_1 = (op_code.first & 0xF) as usize;
                let register_2 = (op_code.second >> 4 & 0xF) as usize;
                self.v[register_1] |= self.v[register_2];
            }
            0x2 => {
                let register_1 = (op_code.first & 0xF) as usize;
                let register_2 = (op_code.second >> 4 & 0xF) as usize;
                self.v[register_1] &= self.v[register_2];
            }
            0x3 => {
                let register_1 = (op_code.first & 0xF) as usize;
                let register_2 = (op_code.second >> 4 & 0xF) as usize;
                self.v[register_1] ^= self.v[register_2];
            }
            0x4 => {
                let register_1 = (op_code.first & 0xF) as usize;
                let register_2 = (op_code.second >> 4 & 0xF) as usize;
                self.v[register_1] = u8::wrapping_add(self.v[register_1], self.v[register_2]);
            }
            0x5 => {
                let register_1 = (op_code.first & 0xF) as usize;
                let register_2 = (op_code.second >> 4 & 0xF) as usize;
                self.v[register_1] = u8::wrapping_sub(self.v[register_1], self.v[register_2]);
            }
            0x6 => {
                let register = (op_code.first & 0xF) as usize;
                let least_significant = get_bit_at(self.v[register], 0);
                self.v[register] >>= 1;
                self.v[0xF] = if least_significant { 1 } else { 0 };
            }
            0x7 => {
                let register_1 = (op_code.first & 0xF) as usize;
                let register_2 = (op_code.second >> 4 & 0xF) as usize;
                self.v[register_1] = u8::wrapping_sub(self.v[register_2], self.v[register_1]);
            }
            0xe => {
                let register = (op_code.first & 0xF) as usize;
                let most_significant = get_bit_at(self.v[register], 7);
                self.v[register] <<= 1;
                self.v[0xF] = if most_significant { 1 } else { 0 };
            }
            _ => println!("unknown op code"),
        };
        self.pc += 2;
        InterpreterResult {
            refresh_display: false,
            wait_for_keyboard: None,
        }
    }

    fn handle_9_op(&mut self, op_code: &OpCode) -> InterpreterResult {
        let register_1 = (op_code.first & 0xF) as usize;
        let register_2 = (op_code.second >> 4 & 0xF) as usize;
        if self.v[register_1] != self.v[register_2] {
            self.pc += 2;
        }
        self.pc += 2;
        InterpreterResult {
            refresh_display: false,
            wait_for_keyboard: None,
        }
    }

    fn handle_a_op(&mut self, op_code: &OpCode) -> InterpreterResult {
        self.i = op_code.to_u16() & 0xfff;
        self.pc += 2;
        InterpreterResult {
            refresh_display: false,
            wait_for_keyboard: None,
        }
    }

    fn handle_b_op(&mut self, op_code: &OpCode) -> InterpreterResult {
        let n = op_code.to_u16() & 0xFFF;
        self.pc += self.v[0] as u16 + n;
        InterpreterResult {
            refresh_display: false,
            wait_for_keyboard: None,
        }
    }

    fn handle_c_op(&mut self, op_code: &OpCode) -> InterpreterResult {
        let register = (op_code.first & 0xF) as usize;
        let r: u8 = random();

        self.v[register] = r & op_code.second;
        self.pc += 2;
        InterpreterResult {
            refresh_display: false,
            wait_for_keyboard: None,
        }
    }

    fn handle_d_op(&mut self, op_code: &OpCode) -> InterpreterResult {
        let mut flipped = false;
        let n = op_code.second & 0xF;
        let x = self.v[(op_code.first & 0xF) as usize];
        let y = self.v[(op_code.second >> 4 & 0xF) as usize];

        for j in 0..n {
            let pos = x as u16 + 64 * (y as u16 + j as u16);

            let sprite_line = self.memory[(self.i + j as u16) as usize].reverse_bits();

            // display mem location pointers
            let mut mem_loc = (DISPLAY_MEM_START + (pos / 8) as usize) % 4096;
            let mut bit_loc = pos % 8;

            // For each bit of sprite
            for k in 0..8 {
                let bit_in_sprite = get_bit_at(sprite_line, k);
                let bit_in_display = get_bit_at(self.memory[mem_loc], bit_loc as u8);
                flipped = flipped || (!bit_in_sprite && bit_in_display);

                let bit = if bit_in_sprite { 1 } else { 0 };

                self.memory[mem_loc] ^= bit << bit_loc;

                bit_loc += 1;
                if bit_loc >= 8 {
                    mem_loc += 1;
                    mem_loc %= 4096;
                    bit_loc = 0;
                }
            }
        }

        self.v[0xF] = if flipped { 1 } else { 0 };
        self.pc += 2;
        InterpreterResult {
            refresh_display: true,
            wait_for_keyboard: None,
        }
    }

    fn handle_e_op(&mut self, op_code: &OpCode) -> InterpreterResult {
        let register = (op_code.first & 0xF) as usize;

        let skip = match op_code.second {
            0x9e => self.keyboard[self.v[register] as usize],
            0xa1 => !self.keyboard[self.v[register] as usize],
            _ => false,
        };

        if skip {
            self.pc += 2;
        }
        self.pc += 2;
        InterpreterResult {
            refresh_display: false,
            wait_for_keyboard: None,
        }
    }

    fn handle_f_op(&mut self, op_code: &OpCode) -> InterpreterResult {
        let register = (op_code.first & 0xF) as usize;
        match op_code.second {
            0x07 => {
                self.v[register] = self.dt;
                self.pc += 2;
            }
            0x0a => {
                self.v[register] = self.dt;
                self.pc += 2;
            }
            0x15 => {
                self.dt = self.v[register];
                self.pc += 2;
            }
            0x18 => {
                self.st = self.v[register];
                self.pc += 2;
            }
            0x1e => {
                let register = (op_code.first & 0xF) as usize;
                self.i = u16::wrapping_add(self.i, self.v[register] as u16);
                self.pc += 2;
            }
            0x29 => {
                let char_i = self.v[register] as u16;
                self.i = FONT_START as u16 + (char_i * 5);
                self.pc += 2;
            }
            0x33 => {
                let mut value = self.v[register];
                self.memory[self.i as usize + 2] = value % 10;
                value /= 10;
                self.memory[self.i as usize + 1] = value % 10;
                value /= 10;
                self.memory[self.i as usize] = value % 10;
                self.pc += 2;
            }
            0x55 => {
                for i in 0..=register {
                    self.memory[self.i as usize + i] = self.v[i];
                }
                self.pc += 2;
            }
            0x65 => {
                for i in 0..=register {
                    self.v[i] = self.memory[self.i as usize + i];
                }
                self.pc += 2;
            }
            _ => panic!("Unknown op code"),
        };
        InterpreterResult {
            refresh_display: false,
            wait_for_keyboard: None,
        }
    }
}
