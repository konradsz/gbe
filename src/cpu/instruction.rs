pub enum Instruction {
    Nop,
    Cp(CpTarget),
    Inc(IncDecTarget),
    Dec(IncDecTarget),
    Add16(AddSource),
}

#[rustfmt::skip]
pub enum CpTarget {
    A, B, C, D, E, H, L, HL, Byte
}

#[rustfmt::skip]
pub enum IncDecTarget {
    A, B, C, D, E, H, L, HL,
}

#[rustfmt::skip]
pub enum AddSource {
    BC, DE, HL, SP,
}

impl Instruction {
    pub fn decode_opcode(opcode: u8) -> Self {
        match opcode {
            0xBF => Instruction::Cp(CpTarget::A),
            0xB8 => Instruction::Cp(CpTarget::B),
            0xB9 => Instruction::Cp(CpTarget::C),
            0xBA => Instruction::Cp(CpTarget::D),
            0xBB => Instruction::Cp(CpTarget::E),
            0xBC => Instruction::Cp(CpTarget::H),
            0xBD => Instruction::Cp(CpTarget::L),
            0xBE => Instruction::Cp(CpTarget::HL),
            0xFE => Instruction::Cp(CpTarget::Byte),
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
