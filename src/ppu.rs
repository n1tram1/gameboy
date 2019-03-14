const VRAM_SIZE: usize = 0x2000;
const VRAM_START_ADDR: u16 = 0x8000;
const SCREEN_W: usize = 160;
const SCREEN_H: usize = 144;

const BG_TILEMAP_SZ: usize = 0x400;
const BG_TILEDATA_SZ: usize = 0x1000;

const TILE_SZ: usize = 16;

use crate::lcd::LCD;

#[derive(Debug)]
enum LCD_Mode {
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
            bgp:  0x00,
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
        }
    }

    fn is_vram_accessible(&self) -> bool {
        let mode = self.get_mode();

        !self.is_lcd_enabled()
            | match mode {
                LCD_Mode::H_Blank | LCD_Mode::V_Blank => true,
                _ => false,
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

    pub fn do_cycle(&mut self) {
        if !self.is_lcd_enabled() {
            return;
        }

        if self.cycles_remaining > 0 {
            self.cycles_remaining -= 1;
            return;
        }

        match self.get_mode() {
            LCD_Mode::OAM_Search => {
                /* TODO: implement OAM */

                /* Just transition into VRAM Transfer. */
                self.set_mode(LCD_Mode::Transfer);

                self.cycles_remaining +=  80;
            },
            LCD_Mode::Transfer   => {
                self.render_line();
                self.set_mode(LCD_Mode::H_Blank);

                self.cycles_remaining += 172;
            },
            LCD_Mode::H_Blank    => {
                if self.ly == 143 {
                    self.set_mode(LCD_Mode::V_Blank);
                } else {
                    self.set_mode(LCD_Mode::OAM_Search);
                }

                self.ly += 1;
                self.cycles_remaining += 204;
            },
            LCD_Mode::V_Blank    => {
                if self.ly > 153 {
                    self.lcd.update();
                    self.ly = 0;
                    self.set_mode(LCD_Mode::OAM_Search);
                } else {
                    self.ly += 1;
                }

                self.cycles_remaining += 456;
            },
        }
    }

    fn render_line(&mut self) {
        let tile_row = ((self.scy + self.ly) / 8) as usize;
        let mut tile_col = (self.scx / 8) as usize;
        let mut tiledata = self.fetch_tile(tile_row, tile_col);

        let mut x = self.scx % 8;
        let y = ((self.scy + self.ly) % 8) as usize * 2;

        for n in 0..160 {
            let lsb = if (tiledata[y] & ( 1 << (7 - x))) > 0 { 1 } else { 0 };
            let msb = if (tiledata[y + 1] & (1 << (7 - x))) > 0 { 2 } else { 0 };
            let color = match lsb + msb {
                0 => 0xffffff,
                _ => 0,
            };

            self.lcd.set_pixel(n, self.ly as usize, color);

            x += 1;
            if x == 8 {
                x = 0;
                tile_col += 1;
                tiledata = self.fetch_tile(tile_col, tile_row);
            }
        }
    }

    fn fetch_tile(&self, col: usize, row: usize) -> Vec<u8> {
        let mapoff = col + row * 32 + self.bg_tilemap_addr() as usize - VRAM_START_ADDR as usize;
        let tiledata_off = (self.vram[mapoff] as usize * 16) + self.tiledata_addr() as usize - VRAM_START_ADDR as usize;

        (&self.vram[tiledata_off..tiledata_off + TILE_SZ]).iter()
                                                          .map(|&b| b)
                                                          .collect()
    }

    fn is_lcd_enabled(&self) -> bool {
        self.lcdc & (1 << 7) > 0
    }

    fn get_mode(&self) -> LCD_Mode {
        match self.stat & 0b11 {
            0 => LCD_Mode::H_Blank,
            1 => LCD_Mode::V_Blank,
            2 => LCD_Mode::OAM_Search,
            3 => LCD_Mode::Transfer,
            _ => panic!("Impossible LCD_Mode reached WTF !"),
        }
    }

    fn set_mode(&mut self, mode: LCD_Mode) {
        /* Clear mode bits. */
        self.stat &= !0b11;

        self.stat |= match mode {
            LCD_Mode::H_Blank => 0,
            LCD_Mode::V_Blank => 1,
            LCD_Mode::OAM_Search => 2,
            LCD_Mode::Transfer => 3,
        };
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
