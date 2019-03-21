use std::collections::HashMap;
use crate::palette::Palette;
use crate::lcd::LCD;

const VRAM_SIZE: usize = 0x2000;
const VRAM_START_ADDR: u16 = 0x8000;
const SCREEN_W: usize = 160;
const SCREEN_H: usize = 144;

const BG_TILEMAP_SZ: usize = 0x400;
const BG_TILEDATA_SZ: usize = 0x1000;

const TILE_SZ: usize = 16;

#[derive(Debug)]
enum LCDMode {
    HBlank,
    VBlank,
    OAMSearch,
    Transfer,
}

pub struct PPU {
    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    bgp: Palette,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
    vram: [u8; VRAM_SIZE],

    lcd: LCD,

    cycles_remaining: usize,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            lcdc: 0x00,
            stat: 0x00,
            scy:  0x00,
            scx:  0x00,
            ly:   0x00,
            lyc:  0x00,
            bgp:  Palette::new(0),
            obp0: 0x00,
            obp1: 0x00,
            wy:   0x00,
            wx:   0x00,
            vram: [0; VRAM_SIZE],

            lcd: LCD::new(SCREEN_W, SCREEN_H),

            cycles_remaining: 0,
    }
}

pub fn read_reg(&self, addr: u16) -> u8 {
    match addr {
        0xFF40 => self.lcdc,
        0xFF41 => self.stat,
        0xFF42 => self.scy,
        0xFF43 => self.scx,
        0xFF44 => self.ly,
        0xFF45 => self.lyc,
        0xFF47 => self.bgp.register,
        0xFF48 => self.obp0,
        0xFF49 => self.obp1,
        0xFF4A => self.wy,
        0xFF4B => self.wx,
        _ => panic!("Invalid memory access on PPU register(addr = {:4X})", addr),
    }
}

pub fn read_vram(&self, addr: u16) -> u8 {
    if self.is_vram_accessible() {
        let index = addr - VRAM_START_ADDR;
        self.vram[index as usize]
    } else {
        0xFF
    }
}

pub fn write_vram(&mut self, addr: u16, val: u8) {
    if self.is_vram_accessible() {
        let index = addr - 0x8000;
        self.vram[index as usize] = val;
    }
}

fn is_vram_accessible(&self) -> bool {
    let mode = self.get_mode();

    match mode {
        LCDMode::HBlank | LCDMode::VBlank => self.is_lcd_disabled(),
        _ => false,
    }
}

fn is_lcd_disabled(&self) -> bool {
    self.lcdc & (1 << 7) == 0
}

pub fn write_reg(&mut self, addr: u16, val: u8) {
    match addr {
        0xFF40 => self.lcdc = val,
        0xFF41 => self.stat = val,
        0xFF42 => self.scy = val,
        0xFF43 => self.scx = val,
        0xFF44 => self.ly = val,
        0xFF45 => self.lyc = val,
        0xFF47 => self.bgp = Palette::new(val),
        0xFF48 => self.obp0 = val,
        0xFF49 => self.obp1 = val,
        0xFF4A => self.wy = val,
        0xFF4B => self.wx = val,
        _ => panic!("Invalid memory access on LCD (addr = {:4X})", addr),
    }
}

pub fn do_cycle(&mut self) {
    if self.is_lcd_disabled() {
        return;
    }

    if self.is_mode_finished() {
        self.cycles_remaining += self.step_through_modes();
    } else {
        self.cycles_remaining -= 1;
    }

}

fn is_mode_finished(&self) -> bool {
    self.cycles_remaining == 0
}

fn step_through_modes(&mut self) -> usize {
    match self.get_mode() {
        LCDMode::OAMSearch => {
            /* TODO: implement OAM */

            /* Just transition into VRAM Transfer. */
            self.set_mode(LCDMode::Transfer);

            80
        },
        LCDMode::Transfer   => {
            self.render_bg_line();
            self.set_mode(LCDMode::HBlank);

            172
        },
        LCDMode::HBlank    => {
            if self.ly == 143 {
                self.set_mode(LCDMode::VBlank);
            } else {
                self.set_mode(LCDMode::OAMSearch);
            }

            self.ly += 1;

            204
        },
        LCDMode::VBlank    => {
            if self.ly > 153 {
                self.lcd.update();
                self.ly = 0;
                self.set_mode(LCDMode::OAMSearch);
            } else {
                self.ly += 1;
            }

            456
        },
    }
}

fn get_mode(&self) -> LCDMode {
    match self.stat & 0b11 {
        0 => LCDMode::HBlank,
        1 => LCDMode::VBlank,
        2 => LCDMode::OAMSearch,
        3 => LCDMode::Transfer,
        _ => panic!("Impossible LCDMode reached WTF !"),
    }
}

fn set_mode(&mut self, mode: LCDMode) {
    /* Clear mode bits. */
    self.stat &= !0b11;

    self.stat |= match mode {
        LCDMode::HBlank => 0,
        LCDMode::VBlank => 1,
        LCDMode::OAMSearch => 2,
        LCDMode::Transfer => 3,
    };
}


fn render_bg_line(&mut self) {
    // let (tile_row, _) = self.scy.overflowing_add(self.ly);
    let (tile_row, _) = self.scy.overflowing_add(self.ly);
    let tile_row = (tile_row / 8) as usize;
    let mut tile_col = (self.scx / 8) as usize;

    let mut tiledata = self.fetch_tile(tile_row, tile_col);

    let mut x = self.scx % 8;
    let y = ((self.scy + self.ly) % 8) as usize * 2;

    for n in 0..160 {
        unsafe {
            let lsb = if ((*tiledata)[y] & ( 1 << (7 - x))) > 0 { 1 } else { 0 };
            let msb = if ((*tiledata)[y + 1] & (1 << (7 - x))) > 0 { 2 } else { 0 };
            let color = self.bgp.to_argb(lsb + msb);

            self.lcd.set_pixel(n, self.ly as usize, color as u32);

            x += 1;
            if x == 8 {
                x = 0;
                tile_col += 1;
                tiledata = self.fetch_tile(tile_col, tile_row);
            }
        }
    }
}

fn fetch_tile(&self, col: usize, row: usize) -> *const [u8] {
    if self.is_bg_enabled() {
        let mapoff = col + row * 32 + self.bg_tilemap_addr() as usize - VRAM_START_ADDR as usize;
        let tiledata_off = (self.vram[mapoff] as usize * 16) + self.tiledata_addr() as usize - VRAM_START_ADDR as usize;

        &self.vram[tiledata_off..tiledata_off + TILE_SZ]
    } else {
        &[0; TILE_SZ]
    }
}

fn is_bg_enabled(&self) -> bool {
    self.lcdc & 1 > 0
}


fn bg_tilemap_addr(&self) -> u16 {
    match self.lcdc & (1 << 3) > 0 {
        false => 0x9800,
        true => 0x9C00,
    }
}

fn tiledata_addr(&self) -> u16 {
    match self.lcdc & (1 << 4) > 0 {
        false => 0x8800,
        true => 0x8000,
    }
}
}
