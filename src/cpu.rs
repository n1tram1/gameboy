use std::path;

use crate::decode;
use crate::mmu::MMU;
use crate::registers::{CpuFlag, Registers};

pub struct CPU {
    registers: Registers,
    mmu: MMU,

    /* Cycle related variables. */
    total_cycles: usize,
    cycles_remaining: u8,

    ime: bool,
}

impl CPU {
    pub fn new(path: &path::Path) -> CPU {
        CPU {
            registers: Registers::new(),
            mmu: MMU::new(path),

            total_cycles: 0,
            cycles_remaining: 0,

            ime: false,
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

        // /* TODO: the ppu shouldn't cycle as fast as the cpu */
        self.mmu.do_cycle();
    }

    fn should_load_next_instr(&self) -> bool {
        self.cycles_remaining == 0
    }

    fn fetch_next_opcode(&self) -> u8 {
        self.mmu.read(self.registers.pc)
    }

    fn execute_instruction(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x00 => {
                /* NOP */
                4
            }
            0x02 => {
                /* LD (BC), A */
                let addr = self.registers.get_bc();
                self.mmu.write(addr, self.registers.a);

                8
            }
            0x04 => {
                /* INC B */
                self.registers.b = self.alu8_inc(self.registers.b);

                4
            }
            0x05 => {
                /* DEC B */
                self.registers.b = self.alu8_dec(self.registers.b);

                4
            }
            0x06 => {
                /* LD B, d8 */
                self.registers.b = self.fetch_imm8();

                8
            },
            0x0C => {
                /* INC C */
                self.registers.c = self.alu8_inc(self.registers.c);

                4
            }
            0x0D => {
                /* DEC C */
                self.registers.c = self.alu8_dec(self.registers.c);

                4
            }
            0x0E => {
                /* LD C, d8 */
                self.registers.c = self.fetch_imm8();

                8
            }
            0x11 => {
                /* LD DE, imm16 */
                let imm16 = self.fetch_imm16();
                self.registers.set_de(imm16);

                12
            }
            0x13 => {
                let de = self.registers.get_de();
                self.registers.set_de(de + 1);

                8
            }
            0x14 => {
                /* INC D */
                self.registers.d = self.alu8_inc(self.registers.d);

                4
            }
            0x15 => {
                /* DEC D */
                self.registers.d = self.alu8_dec(self.registers.d);

                4
            },
            0x16 => {
                /* LD D,d8 */
                self.registers.d = self.fetch_imm8();

                8
            }
            0x17 => {
                /* RLA A */
                self.registers.a = self.alu8_rl(self.registers.a);

                4
            }
            0x18 => {
                /* JR r8 */
                let r8 = self.fetch_imm8() as i8;
                self.registers.pc = CPU::calc_rel_addr(self.registers.pc, r8);

                12
            }
            0x1A => {
                /* LD A,(DE) */
                let val = self.mmu.read(self.registers.get_de());
                self.registers.a = val;

                8
            }
            0x1C => {
                /* INC E */
                self.registers.e = self.alu8_inc(self.registers.e);

                4
            }
            0x1D => {
                /* DEC E */
                self.registers.e -= 1;

                4
            }
            0x1E => {
                /* LD E, d8 */
                self.registers.e = self.fetch_imm8();

                8
            }
            0x1F => {
                /* RRA */
                let carry = match self.registers.a & 1 {
                    1 => true,
                    _ => false,
                };

                self.registers.a >>= 1;

                self.registers.set_flag(CpuFlag::Z, self.registers.a == 0);
                self.registers.set_flag(CpuFlag::N, false);
                self.registers.set_flag(CpuFlag::H, false);
                self.registers.set_flag(CpuFlag::C, carry);

                4
            }
            0x20 => {
                /* JR NZ, r8 */
                let r8 = self.fetch_imm8() as i8;

                if self.registers.get_flag(CpuFlag::Z) == false {
                    self.registers.pc = CPU::calc_rel_addr(self.registers.pc, r8);
                    12
                } else {
                    8
                }
            }
            0x21 => {
                /* LD HL, d16 */
                let d16 = self.fetch_imm16();

                self.registers.set_hl(d16);

                12
            }
            0x22 => {
                /* LD (HL+),A */
                let hl = self.registers.get_hl();
                self.mmu.write(hl, self.registers.a);
                self.registers.set_hl(hl + 1);

                8
            }
            0x23 => {
                /* INC HL */
                self.registers.set_hl(self.registers.get_hl() + 1);

                8
            }
            0x24 => {
                /* INC H */
                self.registers.h = self.alu8_inc(self.registers.h);

                4
            }
            0x25 => {
                /* DEC H */
                self.registers.h = self.alu8_dec(self.registers.h);

                4
            }
            0x28 => {
                /* JR Z, r8 */
                let r8 = self.fetch_imm8() as i8;

                if self.registers.get_flag(CpuFlag::Z) {
                    self.registers.pc = CPU::calc_rel_addr(self.registers.pc, r8);
                    12
                } else {
                    8
                }
            }
            0x2C => {
                /* INC L */
                self.registers.l = self.alu8_inc(self.registers.l);

                4
            }
            0x2D => {
                /* DEC L */
                self.registers.l = self.alu8_dec(self.registers.l);

                4
            }
            0x2E => {
                /* LD L,d8 */
                self.registers.l = self.fetch_imm8();

                8
            }
            0x31 => {
                /* LD SP, d16 */
                self.registers.sp = self.fetch_imm16();

                12
            }
            0x32 => {
                /* LD [HL-], A */
                let addr = self.registers.get_hl();

                self.mmu.write(addr, self.registers.a);
                self.registers.set_hl(addr - 1);

                8
            }
            0x3C => {
                /* INC A */
                self.registers.a = self.alu8_inc(self.registers.a);

                4
            }
            0x3D => {
                /* DEC A */
                self.registers.a = self.alu8_dec(self.registers.a);

                4
            }
            0x3E => {
                /* LD A,d8 */
                self.registers.a = self.fetch_imm8();

                8
            }
            0x4F => {
                /* LD C,A */
                self.registers.c = self.registers.a;

                4
            }
            0x57 => {
                /* LD D,A */
                self.registers.d = self.registers.a;

                4
            }
            0x67 => {
                /* LD H,A */
                self.registers.h = self.registers.a;

                4
            }
            0x77 => {
                /* LD (HL),A */
                self.mmu.write(self.registers.get_hl(), self.registers.a);

                8
            }
            0x7B => {
                /* LD A, E */
                self.registers.a = self.registers.e;

                4
            }
            0x7C => {
                /* LD A,H */
                self.registers.a = self.registers.h;

                4
            }
            0x90 => {
                /* SUB B */
                self.registers.a = self.alu8_sub(self.registers.a, self.registers.b);

                4
            }
            0xAF => {
                /* XOR A */
                self.registers.a = self.alu8_xor(self.registers.a, self.registers.a);

                4
            }
            0xB0 => {
                /* OR B */
                self.registers.a = self.alu8_or(self.registers.a, self.registers.b);

                4
            },
            0xB1 => {
                /* OR C */
                self.registers.a = self.alu8_or(self.registers.a, self.registers.c);

                4
            }
            0xB2 => {
                /* OR D */
                self.registers.a = self.alu8_or(self.registers.a, self.registers.d);

                4
            }
            0xB3 => {
                /* OR E */
                self.registers.a = self.alu8_or(self.registers.a, self.registers.e);

                4
            }
            0xB4 => {
                /* OR H */
                self.registers.a = self.alu8_or(self.registers.a, self.registers.h);

                4
            }
            0xB5 => {
                /* OR L */
                self.registers.a = self.alu8_or(self.registers.a, self.registers.l);

                4
            }
            0xB6 => {
                /* OR (HL) */
                let val = self.mmu.read(self.registers.get_hl());
                self.registers.a = self.alu8_or(self.registers.a, val);

                4
            }
            0xB7 => {
                /* OR A */
                self.registers.a = self.alu8_or(self.registers.a, self.registers.a);

                4
            }
            0xBF => {
                /* CP A */
                let n = self.fetch_imm8();
                self.registers.a = self.alu8_sub(self.registers.a, n);

                4
            }
            0xC1 => {
                /* POP BC */
                let addr = self.pop_word();
                self.registers.set_bc(addr);

                16
            }
            0xC3 => {
                /* JP a16 */
                self.registers.pc = self.fetch_imm16();

                16
            }
            0xC5 => {
                /* PUSH BC */
                self.push_word(self.registers.get_bc());

                16
            }
            0xC9 => {
                /* RET */
                self.registers.pc = self.pop_word();

                8
            }
            0xCB => {
                /* Prefix CB */
                let instr = self.fetch_imm8();
                self.execute_cb_instr(instr)
            }
            0xCD => {
                /* CALL addr */
                let addr = self.fetch_imm16();
                self.push_word(self.registers.pc);

                self.registers.pc = addr;

                24
            },
            0xE0 => {
                /* LDH (a8),A */
                let addr = 0xFF00 + self.fetch_imm8() as u16;
                self.mmu.write(addr, self.registers.a);

                12
            }
            0xE2 => {
                /* LD (C), A */
                self.mmu.write(0xFF00 + self.registers.c as u16, self.registers.a);

                8
            }
            0xEA => {
                let addr = self.fetch_imm16();
                self.mmu.write(addr, self.registers.a);

                16
            }
            0xF0 => {
                /* LDH A,(a8) */
                let addr = 0xFF00 + self.fetch_imm8() as u16;
                self.registers.a = self.mmu.read(addr);

                12
            }
            0xF3 => {
                /* DI */
                /* TODO: the IME needs to be set after the next instruction, this is not what is
                 * going on so far, this is putting the timings off
                 */
                self.ime = false;

                4
            }
            0xF8 => {
                /* EI */
                /* TODO: the IME needs to be set after the next instruction, this is not what is
                 * going on so far, this is putting the timings off
                 */
                self.ime = true;

                4
            }
            0xFE => {
                let d8 = self.fetch_imm8();
                self.alu8_cp(self.registers.a, d8);

                8
            }
            _ => {
                // self.mmu.print_vram();
                self.mmu.do_cycle();
                self.debug_dump();
                panic!(
                    "Unimplemented instructions (opcode = {:2X}) at pc = {:4X}",
                    opcode, self.registers.pc
                );
            }
        }
    }

    fn execute_cb_instr(&mut self, instr: u8) -> u8 {
        match instr {
            0x11 => {
                /* RL C */
                self.registers.c = self.alu8_rl(self.registers.c);

                8
            }
            0x7C => {
                /* Bit 7,H */
                let h = self.registers.h;

                self.registers.set_flag(CpuFlag::Z, (h & (1 << 7)) > 0);
                self.registers.set_flag(CpuFlag::N, false);
                self.registers.set_flag(CpuFlag::H, true);

                8
            }
            _ => panic!("Unimplemented CB instruction {:2X}", instr),
        }
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

    fn calc_rel_addr(addr: u16, r8: i8) -> u16 {
        ((addr as u32 as i32) + (r8 as i32)) as u16
    }

    fn push_word(&mut self, val: u16) {
        self.registers.sp -= 2;
        self.mmu.write_wide(self.registers.sp, val);
    }

    fn pop_word(&mut self) -> u16 {
        let val = self.mmu.read_wide(self.registers.sp);
        self.registers.sp += 2;

        val
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
        let (res, overflow) = a.overflowing_sub(b);

        self.registers.set_flag(CpuFlag::Z, res == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (res & (1 << 4)) > 0); /* TODO: test H flag, it might be very broken */
        self.registers.set_flag(CpuFlag::C, overflow);

        res
    }

    fn alu8_sub_with_borrow(&mut self, a: u8, b: u8) -> u8 {
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

    fn alu8_rl(&mut self, n: u8) -> u8 {
        let old_carry = if self.registers.get_flag(CpuFlag::C) { 1 } else { 0 };
        let new_carry = n & 0x80 == 0x80;
        let res = ((n << 1) & !0b1) | old_carry;

        self.registers.set_flag(CpuFlag::Z, res == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, new_carry);

        res
    }
}
