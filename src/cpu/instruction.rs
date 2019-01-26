pub enum Instruction {
    Load(LoadTarget, u8),
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
pub enum LoadTarget {
    B, C, D, E, H, L,
}

#[rustfmt::skip]
pub enum IncDecTarget {
    A, B, C, D, E, H, L, HL,
}
