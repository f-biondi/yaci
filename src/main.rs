#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::env;
use std::time::{Duration, Instant};
use std::thread::sleep;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use winit::window::{WindowBuilder};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;

mod chip8;
use crate::chip8::Chip8;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() -> Result<(), Error> {
    let args : Vec<String> = env::args().collect();

    let mut system = Chip8::new(); 
    system.load_rom(&args[1]);

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f32 * 10.0, HEIGHT as f32 * 10.0);
        WindowBuilder::new()
            .with_title("YACI")
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

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Left) {
                system.set_key(0x4, true);
            }
            if input.key_released(VirtualKeyCode::Left) {
                system.set_key(0x4, false);
            }
            if input.key_pressed(VirtualKeyCode::Down) {
                system.set_key(0x5, true);
            }
            if input.key_released(VirtualKeyCode::Down) {
                system.set_key(0x5, false);
            }
            if input.key_pressed(VirtualKeyCode::Right) {
                system.set_key(0x6, true);
            }
            if input.key_released(VirtualKeyCode::Right) {
                system.set_key(0x6, false);
            }
        }
        if system.is_awaiting_redraw() {
            system.fulfill_redraw();
            draw(pixels.get_frame(), system.dump_vmem());
            //draw_2(pixels.get_frame(), &system);
            if pixels
                .render()
                    .map_err(|e| panic!("pixels.render() failed: {}", e))
                    .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
            window.request_redraw();
        }
        for i in 0..18 {
            system.clock();
        }
        system.clock_timer();
        sleep(Duration::from_millis(16u64));
    });
}

fn draw(frame: &mut [u8], vmem: &Vec<Vec<u8>>) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i / WIDTH as usize) as usize;
        let y = (i % WIDTH as usize) as usize;

        let rgba = if vmem[x][y] == 1 {
            [0xFF, 0xFF, 0xFF, 0xFF]
        } else {
            [0x00, 0x00, 0x00, 0xff]
        };

        pixel.copy_from_slice(&rgba);
    }
    /*
    let redraw_section = system.get_redraw_section(); 
    for x in redraw_section.0..(redraw_section.0 + redraw_section.2) {
        let line = system.dump_vmem_line(x);
        for y in redraw_section.1..(redraw_section.1 + redraw_section.3) {
            let rgba = if line[y] == 1 {
                [0xFF, 0xFF, 0xFF, 0xFF]
            } else {
                [0x00, 0x00, 0x00, 0xff]
            };
            let start = (256 * x + y) as usize;
            for i in 0..4 {
                frame[start + i] = rgba[i]; 
            }
        }
    }
    */
}

fn draw_2(frame: &mut[u8], system: &Chip8) {
    let redraw_section = system.get_redraw_section(); 
    for x in redraw_section.0..(redraw_section.0 + redraw_section.2) {
        let line = system.dump_vmem_line(x);
        for y in redraw_section.1..(redraw_section.1 + redraw_section.3) {
            let rgba = if line[y] == 1 {
                0xFF
            } else {
                0x00
            };
            let start = (64 * (x*4) + (y*4)) as usize;
            for i in 0..4 {
                frame[start + i] = rgba; 
            }
        }
    }
}
