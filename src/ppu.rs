use std::slice;

const VRAM_SIZE: usize = 0x2000;
const VRAM_START_ADDR: u16 = 0x8000;
const SCREEN_W: usize = 160;
const SCREEN_H: usize = 144;

const BG_TILEMAP_SZ: usize = 0x400;
const BG_TILEDATA_SZ: usize = 0x1000;

const TILE_SZ: usize = 16;

use crate::lcd::LCD;

enum PPU_Mode {
    H_Blank,
    V_Blank,
    OAM_Search,
    Transfer,
}

pub struct PPU {
    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
    vram: [u8; VRAM_SIZE],

    cycles_remaining: usize,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            lcdc: 0x91, /* 0x91 == LCD enabled, 0x90 == LCD disabled */
            stat: 0x00, /* TODO: find init val of this registers. */
            scy: 0x00,
            scx: 0x00,
            ly: 0x00,
            lyc: 0x00,
            bgp: 0xFC,
            obp0: 0xFF,
            obp1: 0xFF,
            wy: 0x00,
            wx: 0x00,
            vram: [0; VRAM_SIZE],

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
            0xFF47 => self.bgp,
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

            println!("wrote val {:2X} to vram at addr {:4X}", val, addr);
        }
    }

    fn is_vram_accessible(&self) -> bool {
        let mode = self.lcd_mode();

        !self.is_lcd_enabled()
            | match mode {
                PPU_Mode::H_Blank | PPU_Mode::V_Blank => true,
                _ => false,
            }
    }

    pub fn print_vram(&self) {
        for (i, el) in self.vram.iter().enumerate() {
            if i % 40 == 0 {
                println!();
            }

            print!("{:2X}, ", el);
        }
    }

    pub fn write_reg(&mut self, addr: u16, val: u8) {
        println!("writing to PPU register");
        match addr {
            0xFF40 => self.lcdc = val,
            0xFF41 => self.stat = val,
            0xFF42 => self.scy = val,
            0xFF43 => self.scx = val,
            0xFF44 => self.ly = val,
            0xFF45 => self.lyc = val,
            0xFF47 => self.bgp = val,
            0xFF48 => self.obp0 = val,
            0xFF49 => self.obp1 = val,
            0xFF4A => self.wy = val,
            0xFF4B => self.wx = val,
            _ => panic!("Invalid memory access on LCD (addr = {:4X})", addr),
        }
    }

    fn lcd_mode(&self) -> PPU_Mode {
        match self.stat & 0b0000_0011 {
            0 => PPU_Mode::H_Blank,
            1 => PPU_Mode::V_Blank,
            2 => PPU_Mode::OAM_Search,
            3 => PPU_Mode::Transfer,
            _ => panic!("Invalid LCD Mode"),
        }
    }

    pub fn do_cycle(&mut self) {
        if !self.is_lcd_enabled() {
            return;
        }

        // let tile = self.fetch_tile(0x4, 0x8);
        // self.print_tile(tile);
        println!("-----------------------------------------");
        self.render_background();
        println!("-----------------------------------------");
    }

    fn render_background(&mut self) {
        let mut lcd = LCD::new(160, 144);
        self.ly = 0;

        while self.ly < 144 {
            let tile_row = ((self.scy + self.ly) / 8) as usize;
            let mut tile_col = (self.scx / 8) as usize;

            let mut tiledata = self.fetch_tile(tile_row, tile_col);
            let mut x = self.scx % 8;
            let y = ((self.scy + self.ly) % 8) as usize;

            for n in 0..160 {
                let lsb = if (tiledata[y] & ( 1 << (7 - x))) > 0 { 1 } else { 0 };
                let msb = if (tiledata[y + 1] & (1 << (7 - x))) > 0 { 2 } else { 0 };
                let color = match lsb + msb {
                    0 => 0xffffff,
                    _ => 0,
                };

                if color == 0 {
                    print!("0");
                } else {
                    print!(" ");
                }

                lcd.set_pixel(n, (self.scy + self.ly) as usize, color);
                lcd.update();

                x += 1;
                if x == 8 {
                    x = 0;
                    tile_col += 1;
                    tiledata = self.fetch_tile(tile_col, tile_row);

                    if (tile_row == 8) && (tile_col == 4) {
                        tile_col = tile_col + 1 - 1;
                    }
                }
            }

            self.ly += 1;
            println!();
        }

        lcd.run_until_escape();
    }

    fn fetch_tile(&self, col: usize, row: usize) -> &[u8] {
        let mapoff = col + row * 32 + self.bg_tilemap_addr() as usize - VRAM_START_ADDR as usize;
        let tiledata_off = (self.vram[mapoff] as usize * 16) + self.tiledata_addr() as usize - VRAM_START_ADDR as usize;

        &self.vram[tiledata_off..tiledata_off + TILE_SZ]
    }

    fn print_tile(&self, tile: &[u8]) {
        println!("------------------------");

        for i in (0..16).step_by(2) {
            let tiledata_low = tile[i];
            let tiledata_high = tile[i + 1];

            for x in 0..8 {
                let mut color = if ((1 << (7 - x)) & tiledata_low) > 0 {
                    1
                } else {
                    0
                };
                color += if ((1 << (7 - x)) & tiledata_high) > 0 {
                    2
                } else {
                    0
                };

                if color == 0 {
                    print!(" ");
                } else {
                    print!("{}", color);
                }
            }

            println!();
        }
    }

    fn renderline(&self) {
        /* Offset of current line into the tilemap. */
        let mapoff = self.bg_tilemap_addr() as usize - VRAM_START_ADDR as usize
            + (((self.scy as u16 + self.ly as u16) as usize % 256) / 8) * 32;

        /* Offset of the current column into the line. */
        let mut lineoff = self.scx as usize / 8;

        /* Offset of the first time into the VRAM */
        let tiledata_offset = self.tiledata_addr() as usize - VRAM_START_ADDR as usize;
        let mut tileoff = self.vram[mapoff + lineoff] as usize + tiledata_offset;

        let mut x = self.scx % 8;

        for i in 0..160 {
            let lsb = if (self.vram[tileoff] & (7 - x)) > 0 {
                1
            } else {
                0
            };
            let msb = if (self.vram[tileoff + 1] & (7 - x)) > 0 {
                2
            } else {
                0
            };
            let color = lsb + msb;

            if color == 0 {
                print!(" ");
            } else {
                print!("{}", color);
            }

            x += 1;

            if x == 8 {
                x = 0;
                lineoff = (lineoff + 1) % 32;
                tileoff = self.vram[mapoff + lineoff] as usize + tiledata_offset;
            }
        }

        println!();
    }


    fn is_lcd_enabled(&self) -> bool {
        self.lcdc & (1 << 7) > 0
    }

    fn get_mode(&self) -> PPU_Mode {
        match self.lcdc & 0b11 {
            0 => PPU_Mode::H_Blank,
            1 => PPU_Mode::V_Blank,
            2 => PPU_Mode::OAM_Search,
            3 => PPU_Mode::Transfer,
            _ => panic!("Impossible PPU_Mode reached WTF !"),
        }
    }

    fn set_mode(&mut self, mode: PPU_Mode) {
        /* Clear mode bits. */
        self.stat &= !0b11;

        self.stat |= match mode {
            PPU_Mode::H_Blank => 0,
            PPU_Mode::V_Blank => 1,
            PPU_Mode::OAM_Search => 2,
            PPU_Mode::Transfer => 3,
        };
    }

    fn bg_tilemap(&self) -> &[u8] {
        let start = (self.bg_tilemap_addr() - VRAM_START_ADDR) as usize;
        let end = start + BG_TILEMAP_SZ;
        &self.vram[start..end]
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
