mod constants;
mod disassembler;
mod interpreter;
mod util;

use crate::constants::DISPLAY_MEM_START;
use crate::interpreter::Interpreter;
use crate::util::get_bit_at;
use pixels::{Error, Pixels, SurfaceTexture};
use std::{env, fs};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, StartCause};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use std::time::{Instant, Duration};

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

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let buffer = fs::read(filename).unwrap();

    let mut interpreter = Interpreter::new(buffer);

    let timer_length = Duration::new(0, 16);

    interpreter.disassemble_program();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                draw(pixels.get_frame(), &interpreter.memory);
                if pixels
                    .render()
                    .map_err(|e| panic!("pixels.render() failed: {:?}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            },
            Event::NewEvents(StartCause::Init) => {
                *control_flow = ControlFlow::WaitUntil(Instant::now() + timer_length);
            }
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                let result = interpreter.tick();
                if result.refresh_display {
                    // Request a redraw
                    window.request_redraw();
                }

                *control_flow = ControlFlow::WaitUntil(Instant::now() + timer_length);
            }
            _ => (),
        };

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
        }
    });
}

fn draw(frame: &mut [u8], memory: &[u8; 4096]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let color = match get_bit_at(memory[DISPLAY_MEM_START + i / 8], (i % 8) as u8) {
            true => WHITE,
            false => BLACK,
        };

        pixel.copy_from_slice(&color);
    }
}
