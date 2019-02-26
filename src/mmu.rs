use std::path;

use crate::cartridge::Cartridge;

pub struct MMU {
    cartridge: Cartridge,
}

impl MMU {
    pub fn new(path: &path::Path) -> MMU {
        MMU {
            cartridge: Cartridge::new(path),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000...0x3FFF => self.cartridge.read(addr),
            0x3FFF...0x7FFF => panic!("Memory banks not supported"),
            0x8000...0x9FFF => 0/* 8KB Video RAM (VRAM) */,
            0xA000...0xBFFF => 0/* 8KB External RAM */,
            0xC000...0xCFFF => 0/* 4KB Work RAM Bank 0 (WRAM) */,
            0xD000...0xDFFF => 0/* 4KB Work RAM Bank 1 (WRAM) */,
            0xE000...0xFDFF => 0/* Same as C000-DDFF (ECHO) */,
            0xFE00...0xFE9F => 0/* Sprite Attribute Table (OAM) */,
            0xFEA0...0xFEFF => 0/* Not Usable */,
            0xFF00...0xFF7F => 0/* I/O Ports */,
            0xFF80...0xFFFE => 0/* High RAM (HRAM) */,
            0xFFFF          => 0/* Interrupt Enable Register */,
            _ => panic!("Out of bounds memory access at addr {}", addr),
        }
    }
}
