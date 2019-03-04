use crate::mmu::MMU;
use crate::registers::Registers;

pub fn decode_instruction(opcode: u8, mmu: &MMU, regs: &Registers) -> String {
    match opcode {
        0x00 => String::from("NOP"),
        0x05 => String::from(format!("DEC B (B = {})", regs.b)),
        0x06 => {
            let d8 = mmu.read(regs.pc + 1);
            String::from(format!("LD B, {:2X}", d8))
        },
        0x32 => {
            String::from(format!("LD [HL-] (HL = {:4X}), A (A = {:2X})", regs.get_hl(), regs.a))
        },
        0x0E => {
            let d8 = mmu.read(regs.pc + 1);
            String::from(format!("LD C, {:2X}", d8))
        },
        0x20 => {
            String::from(format!("LD (BC), A (BC = {:4X}, A = {:2X})", regs.get_bc(), regs.a))
        },
        0x21 => {
            let d16 = mmu.read_wide(regs.pc + 1);
            String::from(format!("LD HL, {:4X}", d16))
        },
        0xAF => {
            let val = regs.a;
            String::from(format!("XOR A (A = {:2X})", val))
        },
        0xC3 => { 
            let a16 = mmu.read_wide(regs.pc + 1);
            String::from(format!("JP a16 {:4X}", a16))
        },
        _ => String::from("NOT IMPLEMENTED IN DECODER")
    }
}
