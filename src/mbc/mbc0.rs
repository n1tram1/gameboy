use crate::mbc::MBC;

pub struct MBC0 {
    rom: Vec<u8>,
}

impl MBC0 {
    pub fn new(rom: Vec<u8>) -> MBC0 {
        MBC0 {
            rom: rom,
        }
    }
}

impl MBC for MBC0 {
    fn read_rom(&self, addr: u16) -> u8 {
        match self.rom.get(addr as usize) {
            Some(byte) => *byte,
            None => panic!("Out of bounds access on MBC0 ROM at address {:4X}", addr),
        }
    }

    fn read_ram(&self, addr: u16) -> u8 {
        0
    }

    fn write_rom(&mut self, addr: u16, val: u8) {
        ()
    }

    fn write_ram(&mut self, addr: u16, val: u8) {
        ()
    }
}
