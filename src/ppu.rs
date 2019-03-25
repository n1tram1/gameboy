use std::collections::HashMap;
use crate::palette::Palette;
use crate::lcd::LCD;

const VIEWPORT_WIDTH: usize = 160;
const VIEWPORT_HEIGHT: usize = 144;
const SCREEN_WIDTH_IN_TILES: usize = 32;

const VRAM_SIZE: usize = 0x2000;
const VRAM_START_ADDR: usize = 0x8000;

const BG_TILEMAP_SZ: usize = 0x400;
const BG_TILEDATA_SZ: usize = 0x1000;

const OAM_SZ: usize = 160;
const OAM_START_ADDR: usize = 0xFE00;

const TILE_SZ: usize = 16;

#[derive(Debug)]
enum LCDMode {
    HBlank, /* Mode 0 */
    VBlank, /* Mode 1 */
    OAMSearch, /* Mode 2 */
    Transfer, /* Mode 3 */
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
    oam: [u8; OAM_SZ],

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
            oam: [0; OAM_SZ],

            lcd: LCD::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT),

            cycles_remaining: 0,
        }
    }

    pub fn get_lcd_ref(&self) -> &LCD {
        &self.lcd
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
            let index = addr as usize - VRAM_START_ADDR;
            self.vram[index]
        } else {
            0xFF
        }
    }

    pub fn write_vram(&mut self, addr: u16, val: u8) {
        if self.is_vram_accessible() {
            let index = addr as usize - VRAM_START_ADDR;
            self.vram[index] = val;
        }
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        if self.is_vram_accessible() {
            let index = addr as usize - OAM_START_ADDR;
            self.oam[index]
        } else {
            0xFF
        }
    }

    pub fn write_oam(&mut self, addr: u16, val: u8) {
        if self.is_vram_accessible() {
            let index = addr as usize - OAM_START_ADDR;
            self.oam[index] = val;
        }
    }

    fn is_vram_accessible(&self) -> bool {
        let mode = self.get_mode();

        match mode {
            LCDMode::HBlank | LCDMode::VBlank | LCDMode::OAMSearch => true,
            _ => self.is_lcd_disabled(),
        }
    }

    fn is_oam_accessible(&self) -> bool {
        let mode = self.get_mode();

        match mode {
            LCDMode::HBlank | LCDMode::VBlank =>true, 
            _ => self.is_lcd_disabled(),
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
            LCDMode::Transfer => {
                self.render_bg_line();
                self.set_mode(LCDMode::HBlank);

                172
            },
            LCDMode::HBlank => {
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
        let tile_row = self.current_tile_row();
        let mut tile_col = self.current_tile_col();

        let mut tiledata = self.fetch_tile(tile_row, tile_col);

        /* Current coordinates in tile being drawn. */
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

    fn current_tile_row(&self) -> usize {
        let (tile_row, _) = self.scy.overflowing_add(self.ly);
        (tile_row / 8) as usize
    }

    fn current_tile_col(&self) -> usize {
        (self.scx  / 8) as usize
    }

    fn fetch_tile(&self, col: usize, row: usize) -> *const [u8] {
        if self.is_bg_enabled() {
            let tiledata_off = self.tile_offset(col, row);

            &self.vram[tiledata_off..tiledata_off + TILE_SZ]
        } else {
            &[0; TILE_SZ]
        }
    }

    fn is_bg_enabled(&self) -> bool {
        self.lcdc & 1 > 0
    }

    fn tile_offset(&self, col: usize, row: usize) -> usize {
        /* Offset of tile number inside the tilemap. */
        let mapoff = col + row * SCREEN_WIDTH_IN_TILES + self.tilemap_offset();

        let tiledata_offset = if self.tiledata_offset()  == 0x8000 {
            /* Tile numbers from 0 to 255 */
            (self.vram[mapoff] as usize * TILE_SZ) + self.tiledata_offset()
        } else {
            /* Tile numbers from -128 to 127 */
            (self.vram[mapoff] as isize * TILE_SZ as isize) as usize + self.tiledata_offset()
        };

        tiledata_offset
    }

    fn tilemap_offset(&self) -> usize {
        let addr = match self.lcdc & (1 << 3) > 0 {
            false => 0x9800,
            true => 0x9C00,
        };

        addr - VRAM_START_ADDR
    }

    fn tiledata_offset(&self) -> usize {
        let addr = match self.lcdc & (1 << 4) > 0 {
            false => 0x9000, /* Indexes are from -128 to 127 => pattern #0 at 0x9000 */
            true => 0x8000,
        };

        addr - VRAM_START_ADDR
    }
}
