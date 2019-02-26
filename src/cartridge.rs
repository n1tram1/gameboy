use std::fs;
use std::path;

use crate::cartridge_info::CartridgeInfo;

const GGB_FLAG_IDX: usize = 0x143;
const CARTRIDGE_TYPE_IDX: usize = 0x147;
const ROM_SIZE_IDX: usize = 0x148;

pub struct Cartridge {
    rom_data: Vec<u8>,
    pub info: CartridgeInfo,
}

impl Cartridge {
    pub fn new(rom_path: &path::Path) -> Cartridge {
        let rom_data = fs::read(rom_path).unwrap();

        let info = CartridgeInfo::new(&rom_data);
        println!("Loaded {}", info.title);

        match rom_data[GGB_FLAG_IDX] {
            0x00 => println!("ROM is for older gameboys"),
            0x80 => println!("ROM compatible with older gameboys"),
            n => panic!("Invalid GDB flag ({}), cannot load this ROM", n),
        }

        print!("Cartridge type: ");
        match rom_data[CARTRIDGE_TYPE_IDX] {
            0x00 => println!("ROM ONLY"),
            n => println!("Unsupported type ({})", n),
        }

        match rom_data[ROM_SIZE_IDX] {
            0x00 => println!("32KByte (no ROM banking)"),
            n => println!("Unsupported ROM size ({} KByte", (0x8000 << n) / 0x400),
        }

        println!("-------------------------");

        Cartridge {
            rom_data: rom_data,
            info: info,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match self.rom_data.get(addr as usize) {
            Some(byte) => *byte,
            None => panic!("Out of bounds access on cartridge at address {}", addr),
        }
    }
}
