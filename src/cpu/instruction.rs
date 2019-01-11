pub enum Instruction {
    Nop,
    Inc(IncDecTarget),
    Dec(IncDecTarget),
    Add16(AddSource),
}

pub enum IncDecTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HL
}

pub enum AddSource {
    BC,
    DE,
    HL,
    SP,
}

impl Instruction {
    pub fn decode_opcode(opcode: u8) -> Self {
        match opcode {
            0x00 => Instruction::Nop,
            0x01 => panic!("{} not implemented", opcode),
            0x02 => panic!("{} not implemented", opcode),
            0x03 => panic!("{} not implemented", opcode),
            0x3C => Instruction::Inc(IncDecTarget::A),
            0x04 => Instruction::Inc(IncDecTarget::B),
            0x0C => Instruction::Inc(IncDecTarget::C),
            0x14 => Instruction::Inc(IncDecTarget::D),
            0x1C => Instruction::Inc(IncDecTarget::E),
            0x24 => Instruction::Inc(IncDecTarget::H),
            0x2C => Instruction::Inc(IncDecTarget::L),
            0x34 => Instruction::Inc(IncDecTarget::HL),
            0x05 => panic!("{} not implemented", opcode),
            0x06 => panic!("{} not implemented", opcode),
            0x07 => panic!("{} not implemented", opcode),
            0x08 => panic!("{} not implemented", opcode),
            0x09 => Instruction::Add16(AddSource::BC),
            0x19 => Instruction::Add16(AddSource::DE),
            0x29 => Instruction::Add16(AddSource::HL),
            0x39 => Instruction::Add16(AddSource::SP),
            0x0A => panic!("{} not implemented", opcode),
            0x0B => panic!("{} not implemented", opcode),
            0x0D => panic!("{} not implemented", opcode),
            0x0E => panic!("{} not implemented", opcode),
            0x0F => panic!("{} not implemented", opcode),
            0x10 => panic!("{} not implemented", opcode),
            0x11 => panic!("{} not implemented", opcode),
            0x12 => panic!("{} not implemented", opcode),
            0x13 => panic!("{} not implemented", opcode),
            0x15 => panic!("{} not implemented", opcode),
            0x16 => panic!("{} not implemented", opcode),
            0x17 => panic!("{} not implemented", opcode),
            0x18 => panic!("{} not implemented", opcode),
            0x1A => panic!("{} not implemented", opcode),
            0x1B => panic!("{} not implemented", opcode),
            0x1D => panic!("{} not implemented", opcode),
            0x1E => panic!("{} not implemented", opcode),
            0x1F => panic!("{} not implemented", opcode),
            0x20 => panic!("{} not implemented", opcode),
            0x21 => panic!("{} not implemented", opcode),
            0x22 => panic!("{} not implemented", opcode),
            0x23 => panic!("{} not implemented", opcode),
            0x25 => panic!("{} not implemented", opcode),
            0x26 => panic!("{} not implemented", opcode),
            0x27 => panic!("{} not implemented", opcode),
            0x28 => panic!("{} not implemented", opcode),
            0x2A => panic!("{} not implemented", opcode),
            0x2B => panic!("{} not implemented", opcode),
            0x2D => panic!("{} not implemented", opcode),
            0x2E => panic!("{} not implemented", opcode),
            0x2F => panic!("{} not implemented", opcode),
            0x30 => panic!("{} not implemented", opcode),
            0x31 => panic!("{} not implemented", opcode),
            0x32 => panic!("{} not implemented", opcode),
            0x33 => panic!("{} not implemented", opcode),
            0x35 => panic!("{} not implemented", opcode),
            0x36 => panic!("{} not implemented", opcode),
            0x37 => panic!("{} not implemented", opcode),
            0x38 => panic!("{} not implemented", opcode),
            0x3A => panic!("{} not implemented", opcode),
            0x3B => panic!("{} not implemented", opcode),
            0x3D => panic!("{} not implemented", opcode),
            0x3E => panic!("{} not implemented", opcode),
            0x3F => panic!("{} not implemented", opcode),
            0x40 => panic!("{} not implemented", opcode),
            0x41 => panic!("{} not implemented", opcode),
            0x42 => panic!("{} not implemented", opcode),
            0x43 => panic!("{} not implemented", opcode),
            0x44 => panic!("{} not implemented", opcode),
            0x45 => panic!("{} not implemented", opcode),
            0x46 => panic!("{} not implemented", opcode),
            0x47 => panic!("{} not implemented", opcode),
            0x48 => panic!("{} not implemented", opcode),
            0x49 => panic!("{} not implemented", opcode),
            0x4A => panic!("{} not implemented", opcode),
            0x4B => panic!("{} not implemented", opcode),
            0x4C => panic!("{} not implemented", opcode),
            0x4D => panic!("{} not implemented", opcode),
            0x4E => panic!("{} not implemented", opcode),
            0x4F => panic!("{} not implemented", opcode),
            0x50 => panic!("{} not implemented", opcode),
            0x51 => panic!("{} not implemented", opcode),
            0x52 => panic!("{} not implemented", opcode),
            0x53 => panic!("{} not implemented", opcode),
            0x54 => panic!("{} not implemented", opcode),
            0x55 => panic!("{} not implemented", opcode),
            0x56 => panic!("{} not implemented", opcode),
            0x57 => panic!("{} not implemented", opcode),
            0x58 => panic!("{} not implemented", opcode),
            0x59 => panic!("{} not implemented", opcode),
            0x5A => panic!("{} not implemented", opcode),
            0x5B => panic!("{} not implemented", opcode),
            0x5C => panic!("{} not implemented", opcode),
            0x5D => panic!("{} not implemented", opcode),
            0x5E => panic!("{} not implemented", opcode),
            0x5F => panic!("{} not implemented", opcode),
            0x60 => panic!("{} not implemented", opcode),
            0x61 => panic!("{} not implemented", opcode),
            0x62 => panic!("{} not implemented", opcode),
            0x63 => panic!("{} not implemented", opcode),
            0x64 => panic!("{} not implemented", opcode),
            0x65 => panic!("{} not implemented", opcode),
            0x66 => panic!("{} not implemented", opcode),
            0x67 => panic!("{} not implemented", opcode),
            0x68 => panic!("{} not implemented", opcode),
            0x69 => panic!("{} not implemented", opcode),
            0x6A => panic!("{} not implemented", opcode),
            0x6B => panic!("{} not implemented", opcode),
            0x6C => panic!("{} not implemented", opcode),
            0x6D => panic!("{} not implemented", opcode),
            0x6E => panic!("{} not implemented", opcode),
            0x6F => panic!("{} not implemented", opcode),
            0x70 => panic!("{} not implemented", opcode),
            0x71 => panic!("{} not implemented", opcode),
            0x72 => panic!("{} not implemented", opcode),
            0x73 => panic!("{} not implemented", opcode),
            0x74 => panic!("{} not implemented", opcode),
            0x75 => panic!("{} not implemented", opcode),
            0x76 => panic!("{} not implemented", opcode),
            0x77 => panic!("{} not implemented", opcode),
            0x78 => panic!("{} not implemented", opcode),
            0x79 => panic!("{} not implemented", opcode),
            0x7A => panic!("{} not implemented", opcode),
            0x7B => panic!("{} not implemented", opcode),
            0x7C => panic!("{} not implemented", opcode),
            0x7D => panic!("{} not implemented", opcode),
            0x7E => panic!("{} not implemented", opcode),
            0x7F => panic!("{} not implemented", opcode),
            0x80 => panic!("{} not implemented", opcode),
            0x81 => panic!("{} not implemented", opcode),
            0x82 => panic!("{} not implemented", opcode),
            0x83 => panic!("{} not implemented", opcode),
            0x84 => panic!("{} not implemented", opcode),
            0x85 => panic!("{} not implemented", opcode),
            0x86 => panic!("{} not implemented", opcode),
            0x87 => panic!("{} not implemented", opcode),
            0x88 => panic!("{} not implemented", opcode),
            0x89 => panic!("{} not implemented", opcode),
            0x8A => panic!("{} not implemented", opcode),
            0x8B => panic!("{} not implemented", opcode),
            0x8C => panic!("{} not implemented", opcode),
            0x8D => panic!("{} not implemented", opcode),
            0x8E => panic!("{} not implemented", opcode),
            0x8F => panic!("{} not implemented", opcode),
            0x90 => panic!("{} not implemented", opcode),
            0x91 => panic!("{} not implemented", opcode),
            0x92 => panic!("{} not implemented", opcode),
            0x93 => panic!("{} not implemented", opcode),
            0x94 => panic!("{} not implemented", opcode),
            0x95 => panic!("{} not implemented", opcode),
            0x96 => panic!("{} not implemented", opcode),
            0x97 => panic!("{} not implemented", opcode),
            0x98 => panic!("{} not implemented", opcode),
            0x99 => panic!("{} not implemented", opcode),
            0x9A => panic!("{} not implemented", opcode),
            0x9B => panic!("{} not implemented", opcode),
            0x9C => panic!("{} not implemented", opcode),
            0x9D => panic!("{} not implemented", opcode),
            0x9E => panic!("{} not implemented", opcode),
            0x9F => panic!("{} not implemented", opcode),
            0xA0 => panic!("{} not implemented", opcode),
            0xA1 => panic!("{} not implemented", opcode),
            0xA2 => panic!("{} not implemented", opcode),
            0xA3 => panic!("{} not implemented", opcode),
            0xA4 => panic!("{} not implemented", opcode),
            0xA5 => panic!("{} not implemented", opcode),
            0xA6 => panic!("{} not implemented", opcode),
            0xA7 => panic!("{} not implemented", opcode),
            0xA8 => panic!("{} not implemented", opcode),
            0xA9 => panic!("{} not implemented", opcode),
            0xAA => panic!("{} not implemented", opcode),
            0xAB => panic!("{} not implemented", opcode),
            0xAC => panic!("{} not implemented", opcode),
            0xAD => panic!("{} not implemented", opcode),
            0xAE => panic!("{} not implemented", opcode),
            0xAF => panic!("{} not implemented", opcode),
            0xB0 => panic!("{} not implemented", opcode),
            0xB1 => panic!("{} not implemented", opcode),
            0xB2 => panic!("{} not implemented", opcode),
            0xB3 => panic!("{} not implemented", opcode),
            0xB4 => panic!("{} not implemented", opcode),
            0xB5 => panic!("{} not implemented", opcode),
            0xB6 => panic!("{} not implemented", opcode),
            0xB7 => panic!("{} not implemented", opcode),
            0xB8 => panic!("{} not implemented", opcode),
            0xB9 => panic!("{} not implemented", opcode),
            0xBA => panic!("{} not implemented", opcode),
            0xBB => panic!("{} not implemented", opcode),
            0xBC => panic!("{} not implemented", opcode),
            0xBD => panic!("{} not implemented", opcode),
            0xBE => panic!("{} not implemented", opcode),
            0xBF => panic!("{} not implemented", opcode),
            0xC0 => panic!("{} not implemented", opcode),
            0xC1 => panic!("{} not implemented", opcode),
            0xC2 => panic!("{} not implemented", opcode),
            0xC3 => panic!("{} not implemented", opcode),
            0xC4 => panic!("{} not implemented", opcode),
            0xC5 => panic!("{} not implemented", opcode),
            0xC6 => panic!("{} not implemented", opcode),
            0xC7 => panic!("{} not implemented", opcode),
            0xC8 => panic!("{} not implemented", opcode),
            0xC9 => panic!("{} not implemented", opcode),
            0xCA => panic!("{} not implemented", opcode),
            0xCB => panic!("{} not implemented", opcode),
            0xCC => panic!("{} not implemented", opcode),
            0xCD => panic!("{} not implemented", opcode),
            0xCE => panic!("{} not implemented", opcode),
            0xCF => panic!("{} not implemented", opcode),
            0xD0 => panic!("{} not implemented", opcode),
            0xD1 => panic!("{} not implemented", opcode),
            0xD2 => panic!("{} not implemented", opcode),
            0xD3 => panic!("{} not implemented", opcode),
            0xD4 => panic!("{} not implemented", opcode),
            0xD5 => panic!("{} not implemented", opcode),
            0xD6 => panic!("{} not implemented", opcode),
            0xD7 => panic!("{} not implemented", opcode),
            0xD8 => panic!("{} not implemented", opcode),
            0xD9 => panic!("{} not implemented", opcode),
            0xDA => panic!("{} not implemented", opcode),
            0xDB => panic!("{} not implemented", opcode),
            0xDC => panic!("{} not implemented", opcode),
            0xDD => panic!("{} not implemented", opcode),
            0xDE => panic!("{} not implemented", opcode),
            0xDF => panic!("{} not implemented", opcode),
            0xE0 => panic!("{} not implemented", opcode),
            0xE1 => panic!("{} not implemented", opcode),
            0xE2 => panic!("{} not implemented", opcode),
            0xE3 => panic!("{} not implemented", opcode),
            0xE4 => panic!("{} not implemented", opcode),
            0xE5 => panic!("{} not implemented", opcode),
            0xE6 => panic!("{} not implemented", opcode),
            0xE7 => panic!("{} not implemented", opcode),
            0xE8 => panic!("{} not implemented", opcode),
            0xE9 => panic!("{} not implemented", opcode),
            0xEA => panic!("{} not implemented", opcode),
            0xEB => panic!("{} not implemented", opcode),
            0xEC => panic!("{} not implemented", opcode),
            0xED => panic!("{} not implemented", opcode),
            0xEE => panic!("{} not implemented", opcode),
            0xEF => panic!("{} not implemented", opcode),
            0xF0 => panic!("{} not implemented", opcode),
            0xF1 => panic!("{} not implemented", opcode),
            0xF2 => panic!("{} not implemented", opcode),
            0xF3 => panic!("{} not implemented", opcode),
            0xF4 => panic!("{} not implemented", opcode),
            0xF5 => panic!("{} not implemented", opcode),
            0xF6 => panic!("{} not implemented", opcode),
            0xF7 => panic!("{} not implemented", opcode),
            0xF8 => panic!("{} not implemented", opcode),
            0xF9 => panic!("{} not implemented", opcode),
            0xFA => panic!("{} not implemented", opcode),
            0xFB => panic!("{} not implemented", opcode),
            0xFC => panic!("{} not implemented", opcode),
            0xFD => panic!("{} not implemented", opcode),
            0xFE => panic!("{} not implemented", opcode),
            0xFF => panic!("{} not implemented", opcode),
            _ => panic!("{} not implemented!", opcode),
        }
    }
}
