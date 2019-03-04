use std::path;

use crate::decode;
use crate::mmu::MMU;
use crate::registers::{Registers, CpuFlag};

pub struct CPU {
    registers: Registers,
    mmu: MMU,

    /* Cycle related variables. */
    total_cycles: usize,
    cycles_remaining: u8,
}

impl CPU {
    pub fn new(path: &path::Path) -> CPU {
        CPU {
            registers: Registers::new(),
            mmu: MMU::new(path),

            total_cycles: 0,
            cycles_remaining: 0,
        }
    }

    fn print_regs(&self) {
        println!("AF {:04X}", self.registers.get_af());
        println!("BC {:04X}", self.registers.get_bc());
        println!("DE {:04X}", self.registers.get_de());
        println!("HL {:04X}", self.registers.get_hl());
        println!("SP {:04X}", self.registers.sp);
        println!("PC {:04X}", self.registers.pc);
    }

    fn debug_dump(&self) {
        println!("\n----- DEBUG DUMP -----");
        self.print_regs();

        println!(
            "total_cycles: {} cycles_remaining: {}",
            self.total_cycles, self.cycles_remaining
        );
    }

    pub fn do_cycle(&mut self) {
        if self.should_load_next_instr() {
            let op = self.fetch_next_opcode();

            println!(
                "op {:2X} pc = {:4X}: \t {}",
                op,
                self.registers.pc,
                decode::decode_instruction(op, &self.mmu, &self.registers)
            );

            /* Go past opcode byte */
            self.registers.pc += 1;

            self.cycles_remaining += self.execute_instruction(op);

            self.total_cycles += self.cycles_remaining as usize;
        } else {
            /* TODO: do something with timing */
            self.cycles_remaining -= 1;
        }
    }

    fn should_load_next_instr(&self) -> bool {
        self.cycles_remaining == 0
    }

    fn fetch_next_opcode(&self) -> u8 {
        self.mmu.read(self.registers.pc)
    }

    fn fetch_imm8(&mut self) -> u8 {
        let byte = self.mmu.read(self.registers.pc);
        self.registers.pc += 1;

        byte
    }

    fn fetch_imm16(&mut self) -> u16 {
        let word = self.mmu.read_wide(self.registers.pc);
        self.registers.pc += 2;

        word
    }

    fn execute_instruction(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x00 => { /* NOP */
                4
            },
            0x05 => { /* DEC B */
                self.registers.b = self.alu8_dec(self.registers.b);

                4
            },
            0x06 => { /* LD B, d8 */
                self.registers.b = self.fetch_imm8();

                8
            },
            0x20 => { /* LD (BC), A */
                let addr = self.registers.get_bc();
                self.mmu.write(addr, self.registers.a);

                8
            },
            0x21 => { /* LD HL, d16 */
                let d16 = self.fetch_imm16();

                self.registers.set_hl(d16);

                12
            },
            0x32 => { /* LD [HL-], A */
                let addr = self.registers.get_hl();

                self.mmu.write(addr, self.registers.a);
                self.registers.set_hl(addr - 1);

                8
            },
            0x0E => { /* LD C, d8 */
                self.registers.c = self.fetch_imm8();

                8
            },
            0xAF => { /* XOR A */
                self.registers.a ^= self.registers.a;

                4
            },
            0xC3 => { /* JP a16 */
                self.registers.pc = self.fetch_imm16();

                16
            },
            _ => {
                self.debug_dump();
                panic!(
                    "Unimplemented instructions (opcode = {:2X}) at pc = {:4X}",
                    opcode, self.registers.pc
                );
            },
        }
    }

    fn alu8_add(&mut self, a: u8, b: u8) -> u8 {
        let (res, overflow) = a.overflowing_add(b);

        self.registers.set_flag(CpuFlag::Z, res == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, (res & (1 << 4)) > 0); /* TODO: test H flag, it might be very broken */
        self.registers.set_flag(CpuFlag::C, overflow);

        res
    }

    fn alu8_adc(&mut self, a: u8, b: u8) -> u8 {
        let carry = match self.registers.get_flag(CpuFlag::C) {
            true => 1,
            false => 0,
        };

        let (res, overflow) = a.overflowing_add(b + carry);

        self.registers.set_flag(CpuFlag::Z, res == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, (res & (1 << 4)) > 0); /* TODO: test H flag, it might be very broken */
        self.registers.set_flag(CpuFlag::C, overflow);

        res
    }

    fn alu8_sub(&mut self, a: u8, b: u8) -> u8 {
        let carry = match self.registers.get_flag(CpuFlag::C) {
            true => 1,
            false => 0,
        };

        let (res, overflow) = a.overflowing_sub(b + carry);

        self.registers.set_flag(CpuFlag::Z, res == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (res & (1 << 4)) > 0); /* TODO: test H flag, it might be very broken */
        self.registers.set_flag(CpuFlag::C, overflow);

        res
    }

    fn alu8_sdc(&mut self, a: u8, b: u8) -> u8 {
        let carry = match self.registers.get_flag(CpuFlag::C) {
            true => 1,
            false => 0,
        };

        let (res, overflow) = a.overflowing_sub(b + carry);

        self.registers.set_flag(CpuFlag::Z, res == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (res & (1 << 4)) > 0); /* TODO: test H flag, it might be very broken */
        self.registers.set_flag(CpuFlag::C, overflow);

        res
    }

    fn alu8_and(&mut self, a: u8, b: u8) -> u8 {
        let res = a & b;

        self.registers.set_flag(CpuFlag::Z, res == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, true);
        self.registers.set_flag(CpuFlag::C, false);

        res
    }

    fn alu8_or(&mut self, a: u8, b: u8) -> u8 {
        let res = a | b;

        self.registers.set_flag(CpuFlag::Z, res == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, false);

        res
    }

    fn alu8_xor(&mut self, a: u8, b: u8) -> u8 {
        let res = a ^ b;

        self.registers.set_flag(CpuFlag::Z, res == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, false);

        res
    }

    fn alu8_cp(&mut self, a: u8, b: u8) {
        let (res, overflow) = a.overflowing_sub(b);

        self.registers.set_flag(CpuFlag::Z, res == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (res & (1 << 4)) > 0); /* TODO: test H flag, it might be very broken */
        self.registers.set_flag(CpuFlag::C, overflow);
    }

    fn alu8_inc(&mut self, n: u8) -> u8 {
        let res = n.wrapping_add(1);

        self.registers.set_flag(CpuFlag::Z, res == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, (n & 0x0F) == 0);

        res
    }
    fn alu8_dec(&mut self, n: u8) -> u8 {
        let res = n.wrapping_sub(1);

        self.registers.set_flag(CpuFlag::Z, res == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (n & 0x0F) == 0);

        res
    }
}
