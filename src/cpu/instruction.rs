#[derive(Debug)]
pub enum Instruction {
    Load(LoadRegister, u8),
    LoadToMemory(u16, LoadRegister),
    LoadToMemoryFromMemory(u16, u8),
    Load16(TargetRegister16, u16),
    LoadStackPointerToMemory(u16),
    PushStack(StackOperationRegisters),
    PopStack(StackOperationRegisters),
    Add8(u8),
    Adc(u8),
    Sub(u8),
    Sbc(u8),
    And(u8),
    Or(u8),
    Xor(u8),
    Cp(u8),
    Inc8(TargetRegister8),
    Dec8(TargetRegister8),
    Inc16(TargetRegister16),
    Dec16(TargetRegister16),
    AddHL(u16),
    Swap(TargetRegister8),
    Cpl,
    Ccf,
    Scf,
    Nop,
    Rlc(TargetRegister8),
    Rl(TargetRegister8),
    Rrc(TargetRegister8),
    Rr(TargetRegister8),
    Sla(TargetRegister8),
    Sra(TargetRegister8),
    Srl(TargetRegister8),
    Bit(TargetRegister8, u8),
    Set(TargetRegister8, u8),
    Res(TargetRegister8, u8),
    Jp,
    Jpcc(bool),
    Jphl,
    Jrn,
    Jrcc(bool),
    Call,
    Callcc(bool),
    Rst(u8),
    Ret,
    Retcc(bool),
}

#[rustfmt::skip]
#[derive(Debug)]
pub enum LoadRegister {
    A, B, C, D, E, H, L
}

#[rustfmt::skip]
#[derive(Debug)]
pub enum TargetRegister8 {
    A, B, C, D, E, H, L, HL,
}

#[rustfmt::skip]
#[derive(Debug)]
pub enum TargetRegister16 {
    BC, DE, HL, SP
}

#[rustfmt::skip]
#[derive(Debug)]
pub enum StackOperationRegisters {
    AF, BC, DE, HL
}
