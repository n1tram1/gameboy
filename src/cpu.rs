use std::path;

use crate::mmu::MMU;
use crate::instructions::{Instruction, INSTRUCTIONS};

pub struct CPU {
    /* Registers */
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,

    /* Instruction related variables. */
    total_cycles: usize,
    instruction: u32,
    cycles_remaining: u8,

    mmu: MMU,
}

impl CPU {
    pub fn new (path: &path::Path) -> CPU {
        /* TODO: check needed initialization values. */
        CPU {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0x100,

            total_cycles: 0,
            instruction: 0,
            cycles_remaining: 0,

            mmu: MMU::new(path),
        }
    }

    pub fn get_af(self) -> u16 { self.af }
    pub fn get_bc(self) -> u16 { self.bc }
    pub fn get_de(self) -> u16 { self.de }
    pub fn get_hl(self) -> u16 { self.hl }

    pub fn set_af(mut self, val: u16) { self.af = val }
    pub fn set_bc(mut self, val: u16) { self.bc = val }
    pub fn set_de(mut self, val: u16) { self.de = val }
    pub fn set_hl(mut self, val: u16) { self.hl = val }

    fn print_regs(&self) {
        println!("AF {:04X}", self.af);
        println!("BC {:04X}", self.bc);
        println!("DE {:04X}", self.de);
        println!("HL {:04X}", self.hl);
        println!("SP {:04X}", self.sp);
        println!("PC {:04X}", self.pc);
    }

    pub fn cycle(&mut self) {
        let op = self.fetch_next_opcode();
        let instr = self.decode_opcode(op);

        println!("{}: {:?}", op, instr);

        self.pc += instr.length;
    }

    fn fetch_next_opcode(&self) -> u8 {
        self.mmu.read(self.pc)
    }

    fn decode_opcode(&self, opcode: u8) -> &Instruction {
        match opcode {
            0x00 => &INSTRUCTIONS[0x00],
            _ => {
                self.print_regs();
                panic!("Unimplemented opcode {:2X} at pc = {}", opcode, self.pc)
            }

        }
    }
}
