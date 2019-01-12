pub enum Instruction {
    Xor(CpXorTarget),
    Cp(CpXorTarget),
    Inc(IncDecTarget),
    Dec(IncDecTarget),
    Add16(AddSource),
    Nop,
}

#[rustfmt::skip]
pub enum CpXorTarget {
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
            0xAF => Instruction::Xor(CpXorTarget::A),
            0xA8 => Instruction::Xor(CpXorTarget::B),
            0xA9 => Instruction::Xor(CpXorTarget::C),
            0xAA => Instruction::Xor(CpXorTarget::D),
            0xAB => Instruction::Xor(CpXorTarget::E),
            0xAC => Instruction::Xor(CpXorTarget::H),
            0xAD => Instruction::Xor(CpXorTarget::L),
            0xAE => Instruction::Xor(CpXorTarget::HL),
            0xEE => Instruction::Xor(CpXorTarget::Byte),
            0xBF => Instruction::Cp(CpXorTarget::A),
            0xB8 => Instruction::Cp(CpXorTarget::B),
            0xB9 => Instruction::Cp(CpXorTarget::C),
            0xBA => Instruction::Cp(CpXorTarget::D),
            0xBB => Instruction::Cp(CpXorTarget::E),
            0xBC => Instruction::Cp(CpXorTarget::H),
            0xBD => Instruction::Cp(CpXorTarget::L),
            0xBE => Instruction::Cp(CpXorTarget::HL),
            0xFE => Instruction::Cp(CpXorTarget::Byte),
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
