use std::{env, io, fs};

struct OpCode {
    first: u8,
    second: u8,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // TODO error handling

    let buffer = fs::read(filename)?;

    let op_codes: Vec<OpCode> = buffer
        .iter()
        .enumerate()
        .step_by(2)
        .map(|(i, byte)| { OpCode { first: *byte, second: buffer[i+1] } })
        .collect();

    for op_code in op_codes.iter() {
        println!("{:02x}{:02x} | {}", op_code.first, op_code.second, handleop(op_code));
    }

    Ok(())
}

fn handleop(op_code: &OpCode) -> String {
    let nibble = op_code.first >> 4 & 0xF;
    match nibble {
        0x0 => handle_0_op(op_code),
        0x1 => handle_1_op(op_code),
        0x2 => handle_2_op(op_code),
        0x3 => handle_3_op(op_code),
        0x4 => handle_4_op(op_code),
        0x5 => handle_5_op(op_code),
        0x6 => handle_6_op(op_code),
        0x7 => handle_7_op(op_code),
        0x8 => handle_8_op(op_code),
        0x9 => handle_9_op(op_code),
        0xa => handle_a_op(op_code),
        0xb => handle_b_op(op_code),
        0xc => handle_c_op(op_code),
        0xd => handle_d_op(op_code),
        0xe => handle_e_op(op_code),
        0xf => handle_f_op(op_code),
        _ => panic!("impossible!"),
    }
}

fn handle_0_op(op_code: &OpCode) -> String {
    match op_code.first & 0xF {
        0 => match op_code.second {
            0xe0 => "CLS".to_string(),
            0xee => "RET".to_string(),
            _ => "Unknown 0".to_string(),
        },
        _ => format!("SYS {:1x}{:2x}", op_code.first & 0xF, op_code.second)
    }.to_owned()
}

fn handle_1_op(op_code: &OpCode) -> String {
    format!("JP {:1x}{:2x}", op_code.first & 0xF, op_code.second)
}

fn handle_2_op(op_code: &OpCode) -> String {
    format!("CALL {:1x}{:2x}", op_code.first & 0xF, op_code.second)
}

fn handle_3_op(op_code: &OpCode) -> String {
    format!("SE v{:1x} {}", op_code.first & 0xF, op_code.second)
}

fn handle_4_op(op_code: &OpCode) -> String {
    format!("SNE v{:1x} {}", op_code.first & 0xF, op_code.second)
}

fn handle_5_op(op_code: &OpCode) -> String {
    format!("SE v{:1x} v{:1x}", op_code.first & 0xF, op_code.second >> 4 & 0xF)
}

fn handle_6_op(op_code: &OpCode) -> String {
    format!("LD v{:1x} {}", op_code.first & 0xF, op_code.second)
}

fn handle_7_op(op_code: &OpCode) -> String {
    format!("ADD v{:1x} {}", op_code.first & 0xF, op_code.second)
}

fn handle_8_op(op_code: &OpCode) -> String {
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

fn handle_9_op(op_code: &OpCode) -> String {
    format!("SNE v{:1x} v{:1x}", op_code.first & 0xF, op_code.second >> 4 & 0xF)
}

fn handle_a_op(op_code: &OpCode) -> String {
    format!("LD I {:01x}{:02x}", op_code.first & 0xF, op_code.second)
}

fn handle_b_op(op_code: &OpCode) -> String {
    format!("JP v0 {:01x}{:02x}", op_code.first & 0xF, op_code.second)
}

fn handle_c_op(op_code: &OpCode) -> String {
    format!("RND v{:1x} {}", op_code.first & 0xF, op_code.second)
}

fn handle_d_op(op_code: &OpCode) -> String {
    format!("DRW v{:1x} v{:1x} {}", op_code.first & 0xF, op_code.second >> 4 & 0xF, op_code.second & 0xF)
}

fn handle_e_op(op_code: &OpCode) -> String {
    match op_code.second {
        0x9e => format!("SKP v{:1x}", op_code.first & 0xF),
        0xa1 => format!("SKNP v{:1x}", op_code.first & 0xF),
        _ => "Unknown e".to_string(),
    }
}

fn handle_f_op(op_code: &OpCode) -> String {
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