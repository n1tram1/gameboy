use std::path;

use crate::mbc;

/* 8kB internal ram */
const INTERNAL_RAM_SIZE: usize = 8182;

pub struct MMU {
    mbc: Box<mbc::MBC>,
    ram: Vec<u8>,
}

impl MMU {
    pub fn new(path: &path::Path) -> MMU {
        MMU {
            mbc: mbc::load_cartridge(path),
            ram: Vec::with_capacity(INTERNAL_RAM_SIZE),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000...0x7FFF => self.mbc.read_rom(addr),
            0x8000...0x9FFF => panic!("NOT IMPLEMENTED"), /* 8KB Video RAM (VRAM) */
            0xA000...0xBFFF => panic!("NOT IMPLEMENTED"), /* 8KB External RAM */
            0xC000...0xCFFF => panic!("NOT IMPLEMENTED"), /* 4KB Work RAM Bank 0 (WRAM) */
            0xD000...0xDFFF => panic!("NOT IMPLEMENTED"), /* 4KB Work RAM Bank 1 (WRAM) */
            0xE000...0xFDFF => panic!("NOT IMPLEMENTED"), /* Same as C000-DDFF (ECHO) */
            0xFE00...0xFE9F => panic!("NOT IMPLEMENTED"), /* Sprite Attribute Table (OAM) */
            0xFEA0...0xFEFF => panic!("NOT IMPLEMENTED"), /* Not Usable */
            0xFF00...0xFF7F => panic!("NOT IMPLEMENTED"), /* I/O Ports */
            0xFF80...0xFFFE => panic!("NOT IMPLEMENTED"), /* High RAM (HRAM) */
            0xFFFF => 0,          /* Interrupt Enable Register */
            _ => panic!("Out of bounds memory access at addr {}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000...0x7FFF => self.mbc.write_rom(addr, value),
            _ => panic!("Unimplemented memory access at addr {:4X}", addr),
        }
    }

    pub fn read_wide(&self, addr: u16) -> u16 {
        (self.read(addr + 1) as u16) << 8 | (self.read(addr) as u16)
    }
}
