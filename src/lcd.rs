extern crate minifb;

use minifb::{Key, Window, WindowOptions};

const LCD_HEIGHT: usize = 160;
const LCD_WIDTH: usize = 144;

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
