use macroquad::prelude::*;

pub struct Screen {
    width: u16,
    height: u16,
    image: Image,
    texture: Texture2D,
}

impl Screen {
    pub fn new(width: u16, height: u16) -> Self {
        let image = Image::gen_image_color(width, height, BLACK); 
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest);
        Self {
            width,
            height,
            image,
            texture,
        }
    }

    pub fn cam_init(&self) {
        let camera : Camera2D = Camera2D::from_display_rect(Rect {
            x: screen_width() / 2.0 - self.width as f32,
            y: screen_height() / 2.0 - self.height as f32,
            w: self.width as f32,
            h: self.height as f32,
        });
        set_camera(&camera);
    }

    pub fn redraw_line(&mut self, line_number: u32,  pixels: &[u8]) {
        for i in 0..self.width {
            self.image.set_pixel(
                i as u32,
                line_number,
                match pixels[i as usize] {
                    1 => WHITE,
                    _ => BLACK,
                },
            );
        }
        self.texture.update(&self.image);
        self.draw();
    }

    pub fn draw(&self) {
        draw_texture(
            self.texture,
            screen_width() / 2.0 - self.width as f32, 
            screen_height() / 2.0 - self.height as f32,
            WHITE
        );
    }
}
