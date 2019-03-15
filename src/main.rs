use std::path;
use std::time::{Duration, Instant};
use std::thread::sleep;

mod cpu;
mod mbc;
mod mmu;
mod registers;
mod ppu;
mod decode;
mod lcd;
mod palette;

fn main() {
    let start = Instant::now();
    let rom_path = path::Path::new("/home/martin/Documents/gameboy-rs/roms/Tetris.GB");

    let mut cpu = cpu::CPU::new(rom_path);

    let mut cycles_count = 0;
    let mut prev = Instant::now();
    let one_sec = Duration::new(1, 0);

    loop {
        cpu.do_cycle();


        if cycles_count > 4 * 10u32.pow(0) {
            println!("elapsed {}ms", start.elapsed().as_micros());


            let diff = Instant::now().duration_since(prev);
            if diff > one_sec {
                println!("running late");
            } else {
                let remaining = one_sec - diff;
                sleep(remaining);
            }


            prev =  Instant::now();
            cycles_count = 0;
        }
    }
}
