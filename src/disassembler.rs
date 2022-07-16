use crate::types::{OpCode, HandleOp};

pub struct Disassembler {}

impl Disassembler {
    fn handle_0_op(&self, op_code: &OpCode) -> String {
        match op_code.first & 0xF {
            0 => match op_code.second {
                0xe0 => "CLS".to_string(),
                0xee => "RET".to_string(),
                _ => "Unknown 0".to_string(),
            },
            _ => format!("SYS {:1x}{:2x}", op_code.first & 0xF, op_code.second)
        }.to_owned()
    }

    fn handle_1_op(&self, op_code: &OpCode) -> String {
        format!("JP {:1x}{:2x}", op_code.first & 0xF, op_code.second)
    }

    fn handle_2_op(&self, op_code: &OpCode) -> String {
        format!("CALL {:1x}{:2x}", op_code.first & 0xF, op_code.second)
    }

    fn handle_3_op(&self, op_code: &OpCode) -> String {
        format!("SE v{:1x} {}", op_code.first & 0xF, op_code.second)
    }

    fn handle_4_op(&self, op_code: &OpCode) -> String {
        format!("SNE v{:1x} {}", op_code.first & 0xF, op_code.second)
    }

    fn handle_5_op(&self, op_code: &OpCode) -> String {
        format!("SE v{:1x} v{:1x}", op_code.first & 0xF, op_code.second >> 4 & 0xF)
    }

    fn handle_6_op(&self, op_code: &OpCode) -> String {
        format!("LD v{:1x} {}", op_code.first & 0xF, op_code.second)
    }

    fn handle_7_op(&self, op_code: &OpCode) -> String {
        format!("ADD v{:1x} {}", op_code.first & 0xF, op_code.second)
    }

    fn handle_8_op(&self, op_code: &OpCode) -> String {
        match op_code.second & 0xF {
            0x0 => format!("LD v{:1x} v{:1x}", op_code.first & 0xF, op_code.second >> 4 & 0xF),
            0x1 => format!("OR v{:1x} v{:1x}", op_code.first & 0xF, op_code.second >> 4 & 0xF),
            0x2 => format!("AND v{:1x} v{:1x}", op_code.first & 0xF, op_code.second >> 4 & 0xF),
            0x3 => format!("XOR v{:1x} v{:1x}", op_code.first & 0xF, op_code.second >> 4 & 0xF),
            0x4 => format!("ADD v{:1x} v{:1x}", op_code.first & 0xF, op_code.second >> 4 & 0xF),
            0x5 => format!("SUB v{:1x} v{:1x}", op_code.first & 0xF, op_code.second >> 4 & 0xF),
            0x6 => format!("SHR v{:1x} v{:1x}", op_code.first & 0xF, op_code.second >> 4 & 0xF),
            0x7 => format!("SUBN v{:1x} v{:1x}", op_code.first & 0xF, op_code.second >> 4 & 0xF),
            0xe => format!("SHL v{:1x} v{:1x}", op_code.first & 0xF, op_code.second >> 4 & 0xF),
            _ => format!("unknown 8"),
        }
    }

    fn handle_9_op(&self, op_code: &OpCode) -> String {
        format!("SNE v{:1x} v{:1x}", op_code.first & 0xF, op_code.second >> 4 & 0xF)
    }

    fn handle_a_op(&self, op_code: &OpCode) -> String {
        format!("LD I {:01x}{:02x}", op_code.first & 0xF, op_code.second)
    }

    fn handle_b_op(&self, op_code: &OpCode) -> String {
        format!("JP v0 {:01x}{:02x}", op_code.first & 0xF, op_code.second)
    }

    fn handle_c_op(&self, op_code: &OpCode) -> String {
        format!("RND v{:1x} {}", op_code.first & 0xF, op_code.second)
    }

    fn handle_d_op(&self, op_code: &OpCode) -> String {
        format!("DRW v{:1x} v{:1x} {}", op_code.first & 0xF, op_code.second >> 4 & 0xF, op_code.second & 0xF)
    }

    fn handle_e_op(&self, op_code: &OpCode) -> String {
        match op_code.second {
            0x9e => format!("SKP v{:1x}", op_code.first & 0xF),
            0xa1 => format!("SKNP v{:1x}", op_code.first & 0xF),
            _ => "Unknown e".to_string(),
        }
    }

    fn handle_f_op(&self, op_code: &OpCode) -> String {
        match op_code.second {
            0x07 => format!("LD v{:1x} DT", op_code.first & 0xF),
            0x0a => format!("LD v{:1x} K", op_code.first & 0xF),
            0x15 => format!("LD DT v{:1x}", op_code.first & 0xF),
            0x18 => format!("LD ST v{:1x}", op_code.first & 0xF),
            0x1e => format!("ADD I v{:1x}", op_code.first & 0xF),
            0x29 => format!("LD F v{:1x}", op_code.first & 0xF),
            0x33 => format!("LD B v{:1x}", op_code.first & 0xF),
            0x55 => format!("LD [I] v{:1x}", op_code.first & 0xF),
            0x65 => format!("LD v{:1x} [I]", op_code.first & 0xF),
            _ => "Unknown f".to_string(),
        }
    }
}

impl HandleOp for Disassembler {
    fn handle_op(&self, op_code: &OpCode) {
        let nibble = op_code.first >> 4 & 0xF;
        let translated = match nibble {
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
        };
        println!("{:02x}{:02x} | {}", op_code.first, op_code.second, translated);
    }
}

