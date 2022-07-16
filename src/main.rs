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
use pixels::{Pixels, SurfaceTexture, Error};
use winit::event::{WindowEvent, Event, VirtualKeyCode};


const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

const WHITE: [u8; 4] = [0xfb, 0xbb, 0xb3, 0xff];
const BLACK: [u8; 4] = [0x88, 0xaa, 0x88, 0xff];

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new((WIDTH * 12) as f64, (HEIGHT * 12) as f64);
        WindowBuilder::new()
            .with_title("CHIP-8")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    // TODO: set up interpreter here
    let op_codes = read_op_codes().unwrap();

    let disassembler = Disassembler {};

    for op_code in op_codes.iter() {
        disassembler.handle_op(op_code);
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::RedrawRequested(_) = event {
            draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| panic!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }


        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Request a redraw
            window.request_redraw();
        }
    });
}

fn read_op_codes() -> io::Result<Vec<OpCode>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let buffer = fs::read(filename)?;

    let res = buffer
        .iter()
        .enumerate()
        .step_by(2)
        .map(|(i, byte)| { OpCode { first: *byte, second: buffer[i+1] } })
        .collect();

    Ok(res)
}

fn draw(frame: &mut [u8]) {
    // TODO this needs to use screen state from Interpreter
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let color =
            if i == 0 {
                WHITE
            } else {
                BLACK
            };

        pixel.copy_from_slice(&color);
    }
}