mod disassembler;
mod types;
mod interpreter;

use std::{env, io, fs};
use crate::types::OpCode;
use crate::disassembler::Disassembler;
use crate::types::HandleOp;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let buffer = fs::read(filename)?;

    let op_codes: Vec<OpCode> = buffer
        .iter()
        .enumerate()
        .step_by(2)
        .map(|(i, byte)| { OpCode { first: *byte, second: buffer[i+1] } })
        .collect();

    let disassembler = Disassembler {};

    for op_code in op_codes.iter() {
        disassembler.handle_op(op_code);
    }

    Ok(())
}