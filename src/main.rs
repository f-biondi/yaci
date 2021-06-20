use std::env;
use std::time::{Duration, Instant};
use std::thread::sleep;
use macroquad::prelude::*;

mod chip8;
mod screen;
use crate::chip8::Chip8;
use crate::screen::Screen;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("YACI"),
        fullscreen: true,
        ..Default::default()
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[macroquad::main(window_conf)]
async fn main() {
    let args : Vec<String> = env::args().collect();

    let mut system = Chip8::new(); 
    system.load_rom(&args[1]);

    let mut screen = Screen::new(64, 32);
    screen.cam_init();

    loop {
        let now = Instant::now();
        //println!("FPS: {}", get_fps());
        system.clock();
        if system.is_awaiting_redraw() {
            for l in 0..32 {
                screen.redraw_line(l as u32, system.dump_vmem_line(l as usize));
            }
            system.fulfill_redraw();
        }
        else {
            screen.draw();
        }
        //handle input
        sleep(Duration::from_millis(16u64  - now.elapsed().as_millis() as u64));
        next_frame().await
    }
}
