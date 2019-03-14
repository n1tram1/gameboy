pub struct Registers {
    pub a: u8,
    f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

pub enum CpuFlag {
    Z = 0b0100_0000,
    N = 0b0010_0000,
    H = 0b0001_0000,
    C = 0b0000_1000,
}

impl Registers {
    pub fn new() -> Registers {
        /* TODO: find the initialization values */
        Registers {
            a:  0x00,
            f:  0x00,
            b:  0x00,
            c:  0x00,
            d:  0x00,
            e:  0x00,
            h:  0x00,
            l:  0x00,
            sp: 0x0000,
            pc: 0x00,
        }
    }

    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | ((self.f & 0xF0) as u16)
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_f(&mut self, value: u8) {
        self.f = value & 0xF0
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = (value as u8) & 0xF0;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }


    pub fn get_flag(&self, flag: CpuFlag) -> bool {
        let mask = flag as u8;
        self.f & mask > 0
    }

    pub fn set_flag(&mut self, flag: CpuFlag, set: bool) {
        let mask = flag as u8;

        match set {
            true => self.f |= mask,
            false => self.f &= !mask,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Registers;
    use super::CpuFlag;

    #[test]
    fn wide_registers() {
        let mut regs = Registers::new();
        regs.a = 0x12;
        regs.set_f(0x23);
        regs.b = 0x34;
        regs.c = 0x45;
        regs.d = 0x56;
        regs.e = 0x67;
        regs.h = 0x78;
        regs.l = 0x89;

        assert_eq!(regs.get_af(), 0x1220);
        assert_eq!(regs.get_bc(), 0x3445);
        assert_eq!(regs.get_de(), 0x5667);
        assert_eq!(regs.get_hl(), 0x7889);

        regs.set_af(0x4242);
        regs.set_bc(0x4242);
        regs.set_de(0x4242);
        regs.set_hl(0x4242);

        assert_eq!(regs.get_af(), 0x4240);
        assert_eq!(regs.get_bc(), 0x4242);
        assert_eq!(regs.get_de(), 0x4242);
        assert_eq!(regs.get_hl(), 0x4242);
    }

    #[test]
    fn flags() {
        let mut regs = Registers::new();

        regs.set_flag(CpuFlag::Z, true);
        regs.set_flag(CpuFlag::N, true);
        regs.set_flag(CpuFlag::H, true);
        regs.set_flag(CpuFlag::C, true);

        assert_eq!(regs.get_flag(CpuFlag::Z), true);
        assert_eq!(regs.get_flag(CpuFlag::N), true);
        assert_eq!(regs.get_flag(CpuFlag::H), true);
        assert_eq!(regs.get_flag(CpuFlag::C), true);

        regs.set_flag(CpuFlag::Z, false);
        regs.set_flag(CpuFlag::N, false);
        regs.set_flag(CpuFlag::H, false);
        regs.set_flag(CpuFlag::C, false);

        assert_eq!(regs.get_flag(CpuFlag::Z), false);
        assert_eq!(regs.get_flag(CpuFlag::N), false);
        assert_eq!(regs.get_flag(CpuFlag::H), false);
        assert_eq!(regs.get_flag(CpuFlag::C), false);
    }
}
