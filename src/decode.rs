use crate::mmu::MMU;
use crate::registers::Registers;

pub fn decode_instruction(opcode: u8, mmu: &MMU, regs: &Registers) -> String {
    match opcode {
        0x00 => String::from("NOP"),
        0x02 => {
            String::from(format!("LD (BC), A (BC = {:4X}, A = {:2X})", regs.get_bc(), regs.a))
        },
        0x05 => String::from(format!("DEC B (B = {})", regs.b)),
        0x06 => {
            let d8 = mmu.read(regs.pc + 1);
            String::from(format!("LD B, {:2X}", d8))
        },
        0x0C => {
            String::from("INC C")
        }
        0x0D => {
            String::from(format!("DEC C (C = {:2X})", regs.c))
        }
        0x0E => {
            let d8 = mmu.read(regs.pc + 1);
            String::from(format!("LD C, {:2X}", d8))
        },
        0x11 => {
            let imm16 = mmu.read_wide(regs.pc + 1);
            String::from(format!("LD DE, {:4X}", imm16))
        }
        0x13 => {
            String::from("INC DE")
        }
        0x14 => {
            String::from("INC D")
        }
        0x15 => {
            String::from("DEC D")
        },
        0x17 => {
            String::from("RLA")
        }
        0x18 => {
            String::from(format!("JR {}", mmu.read(regs.pc + 1) as i8))
        }
        0x1A => {
            String::from("LD A,(DE)")
        }
        0x1F => {
            String::from("RRA")
        },
        0x20 => {
            let r8 = mmu.read(regs.pc + 1);
            String::from(format!("JR NZ, {:2X}", r8))
        },
        0x21 => {
            let d16 = mmu.read_wide(regs.pc + 1);
            String::from(format!("LD HL, {:4X}", d16))
        },
        0x22 => {
            String::from("LD (HL+),A")
        }
        0x23 => {
            String::from("INC HL")
        }
        0x25 => {
            String::from("DEC H")
        }
        0x28 => {
            String::from(format!("JR Z,{}", mmu.read(regs.pc + 1) as i8))
        }
        0x2E => {
            String::from(format!("LD L,{:2X}", mmu.read(regs.pc + 1)))
        }
        0x31 => {
            let d16 = mmu.read_wide(regs.pc + 1);
            String::from(format!("LD SP, {:4X}", d16))
        }
        0x32 => {
            String::from(format!("LD [HL-] (HL = {:4X}), A (A = {:2X})", regs.get_hl(), regs.a))
        },
        0x3E => {
            String::from(format!("LD A, {:2X}", mmu.read(regs.pc + 1)))
        }
        0x3D => {
            String::from("DEC A")
        }
        0x4F => {
            String::from("LD C,A")
        }
        0x77 => {
            String::from("LD (HL),A")
        }
        0x7B => {
            String::from("LD A, E")
        }
        0xAF => {
            let val = regs.a;
            String::from(format!("XOR A (A = {:2X})", val))
        },
        0xB0 => {
            String::from("OR B")
        },
        0xB1 => {
            String::from("OR C")
        }
        0xB2 => {
            String::from("OR D")
        }
        0xB3 => {
            String::from("OR E")
        }
        0xB4 => {
            String::from("OR H")
        }
        0xB5 => {
            String::from("OR L")
        }
        0xB6 => {
            String::from("OR (BC)")
        }
        0xB7 => {
            String::from("OR A")
        }
        0xBF => {
            String::from("CP A")
        }
        0xC1 => {
            String::from("POP BC")
        }
        0xC3 => { 
            let a16 = mmu.read_wide(regs.pc + 1);
            String::from(format!("JP a16 {:4X}", a16))
        },
        0xC5 => {
            String::from("PUSH BC")
        }
        0xC9 => {
            String::from("RET")
        }
        0xCB => {
            let instr = mmu.read(regs.pc + 1);
            String::from(format!("CB {:2X}", instr))
        }
        0xCD => {
            let addr = mmu.read_wide(regs.pc + 1);
            String::from(format!("CALL {:4X}", addr))
        }
        0xE0 => {
            String::from(format!("LDH ($FF00 + {:2X}),A", mmu.read(regs.pc + 1)))
        }
        0xE2 => {
            String::from("LD ($FF00 + C),A")
        }
        0xEA => {
            String::from(format!("LD ({:4X}),A", mmu.read_wide(regs.pc + 1)))
        }
        0xF0 => {
            String::from(format!("LDH A,($FF00 + {:2X})", mmu.read(regs.pc + 1)))
        }
        0xF3 => {
            String::from("DI")
        }
        0xF8 => {
            String::from("EI")
        }
        0xFE => {
            String::from(format!("CP {:2X}", mmu.read(regs.pc + 1)))
        }
        _ => String::from("NOT IMPLEMENTED IN DECODER")
    }
}
