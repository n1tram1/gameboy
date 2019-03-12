extern crate minifb;

use minifb::{Key, Window, WindowOptions};

const LCD_HEIGHT: usize = 160;
const LCD_WIDTH: usize = 144;

pub struct LCD {
    width: usize,
    height: usize,
    pixels: Vec<u32>,
    window: Window,
}

impl LCD {
    pub fn new(width: usize, height: usize) -> LCD {
        let mut lcd = LCD {
            width,
            height,
            pixels: vec![0; width * height],
            window: Window::new(
                "gameboy-rs",
                width,
                height,
                WindowOptions::default(),
            ).unwrap(),
        };

        for n in lcd.pixels.iter_mut() {
            *n = 0x00FFFFFF;
        }
        lcd.update();

        lcd
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, val: u32) {
        self.pixels[x + y * self.width] = val
    }

    pub fn update(&mut self) {
        self.window.update_with_buffer(&self.pixels).unwrap();
    }

    pub fn run_until_escape(&self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {}
    }
}
