use std::path;

mod cpu;
mod mbc;
mod mmu;
mod registers;
mod decode;

fn main() {
    let rom_path = path::Path::new("/home/martin/Documents/gameboy-rs/roms/Tetris.GB");

    let mut cpu = cpu::CPU::new(rom_path);

    loop {
        cpu.do_cycle();
    }
}
