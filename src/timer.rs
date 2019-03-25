pub struct Timer {
    div: u8,
    tima: u8,
    tma: u8,
    tac: u8,
    cycles: usize,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            cycles: 0,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => panic!("Invalid timer registers address {:2X}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF04 => self.div = 0x00,
            0xFF05 => self.tima = val,
            0xFF06 => self.tma = val,
            0xFF07 => self.tac = val,
            _ => panic!("Invalid timer registers address {:2X}", addr),
        }
    }

    pub fn do_cycles(&mut self) {
        if self.cycles % 0xFF  == 0 {
            let (div, _) = self.div.overflowing_add(1);
            self.div = div;
        }

        if self.cycles % self.tac_cycles()  == 0 {
            if self.tima == 0xFF {
                self.tima = self.tma;
            } else {
                let (tima, _) = self.tima.overflowing_add(1);
                self.tima = tima;
            }

        }

        self.cycles += 1;
    }

    fn tac_cycles(&self) -> usize {
        match self.tac {
            0b00 => 1024,
            0b01 => 16,
            0b10 => 64,
            0b11 => 256,
            _ => panic!("Invalid TAC value {:X}", self.tac),
        }
    }
}
