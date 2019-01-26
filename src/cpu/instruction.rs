pub enum Instruction {
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
    Add16(u16),
    Nop,
}

#[rustfmt::skip]
pub enum IncDecTarget {
    A, B, C, D, E, H, L, HL,
}
