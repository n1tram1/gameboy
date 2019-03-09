use std::path;
use std::fs;

mod mbc0;

pub trait MBC {
    fn read_rom(&self, addr: u16) -> u8;
    fn read_ram(&self, addr: u16) -> u8;

    fn write_rom(&mut self, addr: u16, val: u8);
    fn write_ram(&mut self, addr: u16, val: u8);

    fn rom_name(&self) -> String {
        const TITLE_START: u16 = 0x134;
        const CGB_FLAG: u16 = 0x143;

        let title_size = match self.read_rom(CGB_FLAG) {
            0x80 | 0xC0 => 11,
            _ => 16,
        };

        let mut title = String::with_capacity(title_size as usize);

        for i in 0..title_size {
            match self.read_rom(TITLE_START + i) {
                0 => break,
                c => title.push(c as char),
            }
        }

        title
    }
}

pub fn load_cartridge(rom_path: &path::Path) -> Box<MBC> {
    const CARTRIDGE_TYPE_IDX: usize  = 0x147;

    let cartridge_data = fs::read(rom_path).unwrap();

    if cartridge_data.len() < 0x150 {
        eprintln!("ROM too small, can't even fit header ({} bytes)", cartridge_data.len());
    } else {
        println!("Loaded cartridge of size {} bytes\n--------------\n", cartridge_data.len());
    }


    match cartridge_data[CARTRIDGE_TYPE_IDX] {
        0x00 => Box::new(mbc0::MBC0::new(cartridge_data)),
        n => panic!("Unimplemented cartridge type {}", n),
    }
}
