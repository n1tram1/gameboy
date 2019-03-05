use std::path;

use crate::mbc;

/* 8kB internal ram */
const INTERNAL_RAM_SIZE: usize = 8192;

pub struct MMU {
    mbc: Box<mbc::MBC>,
    ram: Vec<u8>,

    interrupt_enable: u8,
    interrupt_flag: u8,
}

impl MMU {
    pub fn new(path: &path::Path) -> MMU {
        MMU {
            mbc: mbc::load_cartridge(path),
            ram: vec![0; INTERNAL_RAM_SIZE],

            interrupt_enable: 0,
            interrupt_flag: 0
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000...0x7FFF => self.mbc.read_rom(addr),
            0x8000...0x9FFF => panic!("NOT IMPLEMENTED"), /* 8KB Video RAM (VRAM) */
            0xA000...0xBFFF => panic!("NOT IMPLEMENTED"), /* 8KB External RAM */
            0xC000...0xDFFF => self.ram[(addr - 0xC000) as usize],   /* 8kB Internal RAM size */
            0xE000...0xFDFF => self.read(addr- 0x2000), /* Same as C000-DDFF (ECHO) */
            0xFE00...0xFE9F => panic!("NOT IMPLEMENTED"), /* Sprite Attribute Table (OAM) */
            0xFEA0...0xFEFF => panic!("NOT IMPLEMENTED"), /* Not Usable */
            0xFF00...0xFF7F => {
                /* I/O Ports */
                match addr {
                    0xFF00...0xFF02 => panic!("Joypad registers not implemented"),
                    0xFF04...0xFF07 => panic!("Timer registers not implemented"),
                    0xFF0F => panic!("Interrupt register not implemented"),
                    0xFF10...0xFF3F => 0, /* Sound I/O Ports, sound not implemented for now. */
                    0xFF40...0xFF4B => panic!("GPU Registers not implemented"),
                    0xFF0F => self.interrupt_flag,
                    _ => panic!("Illegal I/O port address"),
                }
            },
            0xFF80...0xFFFE => panic!("NOT IMPLEMENTED"), /* High RAM (HRAM) */
            0xFFFF => self.interrupt_enable, /* Interrupt Enable Register */
            _ => panic!("Out of bounds memory access at addr {}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000...0x7FFF => self.mbc.write_rom(addr, value),
            0xC000...0xDFFF => self.ram[(addr - 0xC000) as usize] = value,
            0xE000...0xFDFF => self.write(addr - 0x2000, value),
            0xFF00...0xFF7F => {
                /* I/O Ports */
                match addr {
                    0xFF00...0xFF02 => panic!("Joypad registers not implemented"),
                    0xFF04...0xFF07 => panic!("Timer registers not implemented"),
                    0xFF0F => self.interrupt_flag = value,
                    0xFF10...0xFF3F => (), /* Sound I/O Ports, sound not implemented for now. */
                    0xFF40...0xFF4B => panic!("GPU Registers not implemented"),
                    _ => panic!("Illegal I/O port address"),
                }
            },
            0xFFFF => self.interrupt_enable = value,
            _ => panic!("Unimplemented memory access at addr {:4X}", addr),
        }
    }

    pub fn read_wide(&self, addr: u16) -> u16 {
        (self.read(addr + 1) as u16) << 8 | (self.read(addr) as u16)
    }
}
