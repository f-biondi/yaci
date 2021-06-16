use std::time::{Duration, Instant};
use std::thread::sleep;
use macroquad::prelude::*;

mod chip8;
use crate::chip8::Chip8;

#[allow(dead_code)]
#[allow(unused_variables)]
#[macroquad::main("YACI")]
async fn main() {
    let mut system = Chip8::new(); 

    set_camera(&Camera2D {
        rotation: 0.0,
        zoom: Vec2::new(0.03, 0.03),
        target: Vec2::new(screen_width() / 2.0 - 32.0, screen_height() / 2.0 - 16.0),
        offset: Vec2::new(0.0, 0.0),
        render_target: None
    });

    let mut image = Image::gen_image_color(64, 32, BLACK);

    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    loop {
        let now = Instant::now();
        println!("FPS: {}", get_fps());
        system.clock();
        if system.awaiting_redraw() {
            let vram = system.get_vram();
            for i in 0..vram.len() {
                image.set_pixel(
                    (i % 64) as u32,
                    (i / 64) as u32,
                    match vram[i] {
                        1 => WHITE,
                        _ => BLACK,
                    },
                );
            }
            texture.update(&image);
            draw_texture(texture, screen_width() / 2.0 - 64.0, screen_height() / 2.0 - 32.0, LIGHTGRAY);
        }
        sleep(Duration::from_millis(16u64  - now.elapsed().as_millis() as u64));
        next_frame().await
    }
}
