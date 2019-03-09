const VRAM_SIZE: usize = 0x2000;
const SCREEN_W: usize = 160;
const SCREEN_H: usize = 144;

enum PPU_Mode {
    H_Blank,
    V_Blank,
    Searching,
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
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            lcdc: 0x91,
            stat: 0x00, /* TODO: find init val of this registers. */
            scy:  0x00,
            scx:  0x00,
            ly:   0x00,
            lyc:  0x00,
            bgp:  0xFC,
            obp0: 0xFF,
            obp1: 0xFF,
            wy:   0x00,
            wx:   0x00,
            vram: [0; VRAM_SIZE],
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
            0x8000...0x9FFF => self.read_vram(addr),
           _ => panic!("Invalid memory access on LCD (addr = {:4X})", addr),
        }
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        if self.is_vram_accessible()  {
            let index = addr - 0x8000;
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
        let mode = self.lcd_mode();

        match mode {
            PPU_Mode::H_Blank |
            PPU_Mode::V_Blank => true,
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
            2 => PPU_Mode::Searching,
            3 => PPU_Mode::Transfer,
            _ => panic!("Invalid LCD Mode"),
        }
    }
}
