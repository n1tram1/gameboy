extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use crate::joypad;

const LCD_HEIGHT: usize = 160;
const LCD_WIDTH: usize = 144;

/* Colors encoded using ARGB format */
pub enum Colors {
    Black     = 0x00_00_00_00,
    White     = 0x00_FF_FF_FF,
    LightGray = 0x00_D3_D3_D3,
    DarkGray  = 0x00_A9_A9_A9,
}

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

        lcd.reset();
        lcd.update();

        lcd
    }

    pub fn reset(&mut self) {
        for n in self.pixels.iter_mut() {
            *n = Colors::White as u32;
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, val: u32) {
        self.pixels[x + y * self.width] = val
    }

    pub fn update(&mut self) {
        self.window.update_with_buffer(&self.pixels).unwrap();
    }

    pub fn run_until_escape(&self) {
        while self.window.is_open() {
            if self.window.is_key_down(Key::Escape) {
                break;
            }
        }
    }

    pub fn get_key(&self, key: joypad::Keys) -> bool {
        let minifb_key = match key {
            joypad::Keys::A => Key::W,
            joypad::Keys::B => Key::Q,
            joypad::Keys::Select => Key::A,
            joypad::Keys::Start => Key::S,
            joypad::Keys::Left => Key::Left,
            joypad::Keys::Right => Key::Right,
            joypad::Keys::Up => Key::Up,
            joypad::Keys::Down => Key::Down,
        };

        self.window.is_key_down(minifb_key)
    }
}
