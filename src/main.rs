mod disassembler;
mod types;
mod interpreter;

use std::{env, io, fs};
use crate::types::OpCode;
use crate::disassembler::Disassembler;
use crate::types::HandleOp;
use winit_input_helper::WinitInputHelper;
use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use pixels::{Pixels, SurfaceTexture};
use winit::event::{WindowEvent, Event};


const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

fn main() -> io::Result<()> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("CHIP-8")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_max_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

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

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}