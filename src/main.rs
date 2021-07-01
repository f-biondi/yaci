use std::env;
use std::time::{Duration};
use std::thread::sleep;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use winit::window::{Window, WindowBuilder};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;

mod chip8;
use crate::chip8::Chip8;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const SCALING: f32 = 10.0;
const KEY_CODES: [VirtualKeyCode; 16] = [
    VirtualKeyCode::X,
    VirtualKeyCode::Key1,
    VirtualKeyCode::Key2,
    VirtualKeyCode::Key3,
    VirtualKeyCode::Q,
    VirtualKeyCode::W,
    VirtualKeyCode::E,
    VirtualKeyCode::A,
    VirtualKeyCode::S,
    VirtualKeyCode::D,
    VirtualKeyCode::Z,
    VirtualKeyCode::C,
    VirtualKeyCode::Key4,
    VirtualKeyCode::R,
    VirtualKeyCode::F,
    VirtualKeyCode::V,
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args : Vec<String> = env::args().collect();

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        //let size = LogicalSize::new(591, 295);
        let size = LogicalSize::new(WIDTH as f32 * SCALING, HEIGHT as f32 * SCALING);
        WindowBuilder::new()
            .with_title("YACI")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)?
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut system = Chip8::new(); 
    system.load_rom(&args[1]);

    event_loop.run(move |event, _, control_flow| {
        system.cycle(18, pixels.get_frame());
        if system.awaiting_redraw() {
            if pixels
                .render()
                    .map_err(|e| panic!("pixels.render() failed: {}", e))
                    .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
            window.request_redraw();
            system.fulfill_redraw();
        }
        if input.update(&event) {
            for (i, key) in KEY_CODES.iter().enumerate() {
                if input.key_pressed(*key) {
                    system.set_key(i as u8, true);
                }
                if input.key_released(*key) {
                    system.set_key(i as u8, false);
                }
            }
        }
        sleep(Duration::from_millis(16));
    });
}
