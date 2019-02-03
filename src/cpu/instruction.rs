pub enum Instruction {
    Load(LoadRegister, u8),
    LoadToMemory(u16, LoadRegister),
    LoadToMemoryFromMemory(u16, u8),
    Add8(u8),
    Adc(u8),
    Sub(u8),
    Sbc(u8),
    And(u8),
    Or(u8),
    Xor(u8),
    Cp(u8),
    Inc(IncDecTarget),
    Dec(IncDecTarget),
    AddHL(u16),
    Nop,
}

#[rustfmt::skip]
pub enum LoadRegister {
    A, B, C, D, E, H, L
}

#[rustfmt::skip]
pub enum IncDecTarget {
    A, B, C, D, E, H, L, HL,
}
