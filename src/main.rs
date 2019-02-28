use std::path;

mod cartridge;
mod cartridge_info;
mod cpu;
mod instructions;
mod mmu;
mod registers;
mod decode;

fn main() {
    let rom_path = path::Path::new("/home/martin/Documents/gameboy-rs/Tetris.GB");

    let mut cpu = cpu::CPU::new(rom_path);

    loop {
        cpu.do_cycle();
    }
}
