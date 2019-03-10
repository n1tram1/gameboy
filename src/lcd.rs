extern crate minifb;

use minifb::{Key, Window, WindowOptions};

const VIEWPORT_HEIGHT: usize = 160;
const VIEWPORT_WIDTH: usize = 144;

const LCD_BUF_HEIGHT: usize = 256;
const LCD_BUF_WIDTH: usize = 256;

struct LCD {
    window: Window,
    pixels: [[u32; LCD_BUF_WIDTH]; LCD_BUF_HEIGHT],
}

impl LCD {
    pub fn new() -> LCD {
        LCD {
            window: Window::new(
                "gameboy-rs",
                VIEWPORT_HEIGHT,
                VIEWPORT_WIDTH,
                WindowOptions::default().unwrap(),
            ),
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, val: u32) {
        self.pixels[y][x] = val
    }

    pub fn update(&self) {
        self.window.update_with_buffer(self.pixels)
    }
}
