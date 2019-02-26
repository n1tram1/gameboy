#[derive(Debug)]
pub struct Instruction {
    pub name: &'static str,
    pub length: u16,
    pub cycles: u8,
}

pub static INSTRUCTIONS: &'static [Instruction] = &[
    Instruction {name: "NOP", length: 1, cycles: 4},
    Instruction {name: "LD BC, d16", length: 3, cycles: 12},
];
