use std::path;

mod cpu;
mod instructions;
mod mmu;
mod cartridge;
mod cartridge_info;

fn main() {
    let rom_path = path::Path::new("/home/martin/Documents/gameboy-rs/Tetris.GB");

    let mut cpu = cpu::CPU::new(rom_path);
    cpu.cycle();
    cpu.cycle();
}
