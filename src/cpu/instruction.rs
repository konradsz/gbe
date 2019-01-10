pub enum Instruction {
    Add16(AddSource)
}

pub enum AddSource {
    BC
}

impl Instruction {
    pub fn decode_opcode(opcode: u8) -> Self {
        match opcode {
            0x09 => Instruction::Add16(AddSource::BC),
            _ => panic!("{} not implemented!", opcode)
        }
    }
}
