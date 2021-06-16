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
        set_camera(&Camera2D {
            rotation: 0.0,
            zoom: Vec2::new(0.03, 0.03),
            target: Vec2::new(
                screen_width() / 2.0 - (self.width as f32) / 2.0,
                screen_height() / 2.0 - (self.height as f32) / 2.0
            ),
            offset: Vec2::new(0.0, 0.0),
            render_target: None
        });
    }

    pub fn redraw(&mut self, pixels: &[u8]) {
        for i in 0..pixels.len() {
            self.image.set_pixel(
                (i % 64) as u32,
                (i / 64) as u32,
                match pixels[i] {
                    1 => WHITE,
                    _ => BLACK,
                },
            );
        }
        self.texture.update(&self.image);
        draw_texture(
            self.texture,
            screen_width() / 2.0 - self.width as f32, 
            screen_height() / 2.0 - self.height as f32,
            WHITE
        );
    }
}
