pub enum Instruction {
    Or(RegistersByteTarget),
    Xor(RegistersByteTarget),
    Cp(RegistersByteTarget),
    Inc(IncDecTarget),
    Dec(IncDecTarget),
    Add16(AddSource),
    Nop,
}

#[rustfmt::skip]
pub enum RegistersByteTarget {
    A, B, C, D, E, H, L, HL, Byte
}

#[rustfmt::skip]
pub enum IncDecTarget { // Registers8Target
    A, B, C, D, E, H, L, HL,
}

#[rustfmt::skip]
pub enum AddSource { // Registers16Target
    BC, DE, HL, SP,
}

impl Instruction {
    pub fn decode_opcode(opcode: u8) -> Self {
        match opcode {
            0xB7 => Instruction::Or(RegistersByteTarget::A),
            0xB0 => Instruction::Or(RegistersByteTarget::B),
            0xB1 => Instruction::Or(RegistersByteTarget::C),
            0xB2 => Instruction::Or(RegistersByteTarget::D),
            0xB3 => Instruction::Or(RegistersByteTarget::E),
            0xB4 => Instruction::Or(RegistersByteTarget::H),
            0xB5 => Instruction::Or(RegistersByteTarget::L),
            0xB6 => Instruction::Or(RegistersByteTarget::HL),
            0xF6 => Instruction::Or(RegistersByteTarget::Byte),
            0xAF => Instruction::Xor(RegistersByteTarget::A),
            0xA8 => Instruction::Xor(RegistersByteTarget::B),
            0xA9 => Instruction::Xor(RegistersByteTarget::C),
            0xAA => Instruction::Xor(RegistersByteTarget::D),
            0xAB => Instruction::Xor(RegistersByteTarget::E),
            0xAC => Instruction::Xor(RegistersByteTarget::H),
            0xAD => Instruction::Xor(RegistersByteTarget::L),
            0xAE => Instruction::Xor(RegistersByteTarget::HL),
            0xEE => Instruction::Xor(RegistersByteTarget::Byte),
            0xBF => Instruction::Cp(RegistersByteTarget::A),
            0xB8 => Instruction::Cp(RegistersByteTarget::B),
            0xB9 => Instruction::Cp(RegistersByteTarget::C),
            0xBA => Instruction::Cp(RegistersByteTarget::D),
            0xBB => Instruction::Cp(RegistersByteTarget::E),
            0xBC => Instruction::Cp(RegistersByteTarget::H),
            0xBD => Instruction::Cp(RegistersByteTarget::L),
            0xBE => Instruction::Cp(RegistersByteTarget::HL),
            0xFE => Instruction::Cp(RegistersByteTarget::Byte),
            0x3C => Instruction::Inc(IncDecTarget::A),
            0x04 => Instruction::Inc(IncDecTarget::B),
            0x0C => Instruction::Inc(IncDecTarget::C),
            0x14 => Instruction::Inc(IncDecTarget::D),
            0x1C => Instruction::Inc(IncDecTarget::E),
            0x24 => Instruction::Inc(IncDecTarget::H),
            0x2C => Instruction::Inc(IncDecTarget::L),
            0x34 => Instruction::Inc(IncDecTarget::HL),
            0x3D => Instruction::Dec(IncDecTarget::A),
            0x05 => Instruction::Dec(IncDecTarget::B),
            0x0D => Instruction::Dec(IncDecTarget::C),
            0x15 => Instruction::Dec(IncDecTarget::D),
            0x1D => Instruction::Dec(IncDecTarget::E),
            0x25 => Instruction::Dec(IncDecTarget::H),
            0x2D => Instruction::Dec(IncDecTarget::L),
            0x35 => Instruction::Dec(IncDecTarget::HL),
            0x09 => Instruction::Add16(AddSource::BC),
            0x19 => Instruction::Add16(AddSource::DE),
            0x29 => Instruction::Add16(AddSource::HL),
            0x39 => Instruction::Add16(AddSource::SP),
            0x00 => Instruction::Nop,
            _ => panic!("{} not implemented!", opcode),
        }
    }
}
