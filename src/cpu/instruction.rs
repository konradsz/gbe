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
    Inc8(IncDecTarget),
    Dec8(IncDecTarget),
    Inc16(TargetRegister16),
    Dec16(TargetRegister16),
    AddHL(u16),
    Swap(IncDecTarget),
    Cpl,
    Ccf,
    Scf,
    Nop,
    Rlc(IncDecTarget),
    Rl(IncDecTarget),
    Rrc(IncDecTarget),
    Rr(IncDecTarget),
    Sla(IncDecTarget),
    Sra(IncDecTarget),
    Srl(IncDecTarget),
    Bit(IncDecTarget, u8),
    Set(IncDecTarget, u8),
    Res(IncDecTarget, u8),
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
pub enum LoadRegister {
    A, B, C, D, E, H, L
}

#[rustfmt::skip]
pub enum TargetRegister16 {
    BC, DE, HL, SP
}

#[rustfmt::skip]
pub enum StackOperationRegisters {
    AF, BC, DE, HL
}


// rename, used also in Swap, RLC, RL, RRC, RR
#[rustfmt::skip]
pub enum IncDecTarget {
    A, B, C, D, E, H, L, HL,
}
